use std::{
    collections::HashSet,
    env,
    error::Error,
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
};

use ructe::Ructe;
use typst::{
    diag::{FileError, FileResult, SourceDiagnostic},
    foundations::{Array, Bytes, Datetime, Dict, Label, Str, Value},
    introspection::{Introspector, MetadataElem},
    model::Document,
    syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot},
    text::{Font, FontBook},
    utils::LazyHash,
    Feature, Library, LibraryExt, World,
};
use typst_html::HtmlDocument;
use typst_layout::PagedDocument;

fn main() -> Result<(), Box<dyn Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let output_dir = PathBuf::from(env::var("OUT_DIR")?);
    let cv_source = manifest_dir.join("cv/cv.typ");
    let cv_pdf = output_dir.join("cv.pdf");
    let math_font = output_dir.join("new-cm-math-regular.otf");
    let blog_dir = manifest_dir.join("blog");
    let posts_rs = output_dir.join("posts.rs");

    assert!(manifest_dir.is_absolute());
    assert!(output_dir.is_absolute());

    println!("cargo:rerun-if-changed={}", cv_source.display());
    println!("cargo:rerun-if-changed={}", blog_dir.display());
    Ructe::from_env()?.compile_templates("templates")?;
    compile_cv(&cv_source, &cv_pdf)?;
    write_math_font(&math_font)?;
    compile_posts(&blog_dir, &posts_rs)?;

    assert!(cv_pdf.is_file());
    assert!(math_font.is_file());
    assert!(posts_rs.is_file());
    assert!(fs::metadata(cv_pdf)?.len() > 1_024);
    assert!(fs::metadata(math_font)?.len() > 100_000);
    assert!(fs::metadata(posts_rs)?.len() > 1_024);
    Ok(())
}

fn compile_cv(source_path: &Path, output_path: &Path) -> Result<(), Box<dyn Error>> {
    assert!(source_path.is_file());
    assert_eq!(
        source_path
            .extension()
            .and_then(|extension| extension.to_str()),
        Some("typ")
    );

    let source = fs::read_to_string(source_path)?;
    assert!(source.contains("Vilhelm Bergsøe"));
    assert!(!source.is_empty());

    let world = SiteWorld::new("cv.typ", source);
    let compiled = typst::compile::<PagedDocument>(&world);
    emit_warnings("CV", &compiled.warnings);

    let document = compiled
        .output
        .map_err(|diagnostics| format_diagnostics("Typst CV compilation failed", &diagnostics))?;
    let pdf = typst_pdf::pdf(&document, &typst_pdf::PdfOptions::default())
        .map_err(|diagnostics| format_diagnostics("Typst PDF export failed", &diagnostics))?;

    assert!(pdf.starts_with(b"%PDF-"));
    assert!(pdf.len() > 1_024);
    fs::write(output_path, pdf)?;

    let written = fs::read(output_path)?;
    assert!(written.starts_with(b"%PDF-"));
    assert!(written.len() > 1_024);
    Ok(())
}

fn write_math_font(output_path: &Path) -> Result<(), Box<dyn Error>> {
    assert_eq!(
        output_path
            .extension()
            .and_then(|extension| extension.to_str()),
        Some("otf")
    );
    assert!(output_path.is_absolute());

    let font = typst_assets::fonts()
        .flat_map(|data| Font::iter(Bytes::new(data)))
        .find(|font| font.post_script_name().as_deref() == Some("NewCMMath-Regular"))
        .ok_or_else(|| io_error("typst-assets does not contain NewCMMath-Regular"))?;

    assert_eq!(font.info().family, "New Computer Modern Math");
    assert!(font.data().len() > 100_000);
    fs::write(output_path, font.data())?;

    let written = fs::read(output_path)?;
    assert_eq!(written.len(), font.data().len());
    assert!(written.len() > 100_000);
    Ok(())
}

fn compile_posts(blog_dir: &Path, output_path: &Path) -> Result<(), Box<dyn Error>> {
    assert!(blog_dir.is_dir());
    assert_eq!(
        output_path
            .extension()
            .and_then(|extension| extension.to_str()),
        Some("rs")
    );

    let mut paths = Vec::new();
    for entry in fs::read_dir(blog_dir)? {
        let path = entry?.path();
        if path.extension().and_then(|extension| extension.to_str()) == Some("typ") {
            paths.push(path);
        }
    }
    paths.sort();

    assert!(!paths.is_empty());
    assert!(paths.iter().all(|path| path.is_file()));

    let mut posts = Vec::with_capacity(paths.len());
    let mut slugs = HashSet::with_capacity(paths.len());
    for path in paths {
        println!("cargo:rerun-if-changed={}", path.display());
        let post = compile_post(&path)?;
        if !slugs.insert(post.slug.clone()) {
            return Err(io_error(format!("duplicate post slug: {}", post.slug)).into());
        }
        posts.push(post);
    }

    posts.sort_by(|left, right| right.date.cmp(&left.date));
    assert_eq!(posts.len(), slugs.len());
    assert!(posts.windows(2).all(|pair| pair[0].date >= pair[1].date));

    write_generated_posts(output_path, &posts)?;
    assert!(output_path.is_file());
    assert!(fs::metadata(output_path)?.len() > 1_024);
    Ok(())
}

fn compile_post(source_path: &Path) -> Result<CompiledPost, Box<dyn Error>> {
    assert!(source_path.is_file());
    assert_eq!(
        source_path
            .extension()
            .and_then(|extension| extension.to_str()),
        Some("typ")
    );

    let slug = source_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| io_error(format!("invalid post filename: {}", source_path.display())))?
        .to_owned();
    let source = fs::read_to_string(source_path)?;

    assert!(!slug.is_empty());
    assert!(source.contains("<post-meta>"));

    let virtual_path = format!("blog/{slug}.typ");
    let world = SiteWorld::new(&virtual_path, source.clone());
    let compiled = typst::compile::<HtmlDocument>(&world);
    emit_warnings(&slug, &compiled.warnings);
    let document = compiled.output.map_err(|diagnostics| {
        format_diagnostics(
            &format!("Typst post compilation failed ({slug})"),
            &diagnostics,
        )
    })?;

    let metadata = post_metadata(&document, &slug)?;
    let encoded = typst_html::html(&document, &typst_html::HtmlOptions::default()).map_err(
        |diagnostics| {
            format_diagnostics(&format!("Typst HTML export failed ({slug})"), &diagnostics)
        },
    )?;
    let content = extract_article_html(&encoded)?;

    assert!(!metadata.title.is_empty());
    assert!(!content.is_empty());
    assert_eq!(
        document.info().title.as_deref(),
        Some(metadata.title.as_str())
    );
    assert_eq!(document.info().keywords.len(), metadata.tags.len());

    Ok(CompiledPost {
        slug,
        title: metadata.title,
        date: metadata.date,
        archived: metadata.archived,
        tags: metadata.tags,
        content,
        estimated_read_time: source.split_whitespace().count() / 200,
    })
}

fn post_metadata(document: &HtmlDocument, slug: &str) -> Result<PostMetadata, Box<dyn Error>> {
    assert!(!slug.is_empty());
    assert!(document.root().children.len() >= 2);

    let label = Label::construct(Str::from("post-meta"))
        .map_err(|error| io_error(format!("invalid metadata label: {error}")))?;
    let content = document
        .introspector()
        .query_label(label)
        .map_err(|error| io_error(format!("invalid post metadata ({slug}): {error}")))?;
    let element = content
        .to_packed::<MetadataElem>()
        .ok_or_else(|| io_error(format!("post metadata is not a metadata element ({slug})")))?;
    let dictionary = match &element.value {
        Value::Dict(dictionary) => dictionary,
        value => {
            return Err(io_error(format!(
                "post metadata must be a dictionary ({slug}), got {}",
                value.ty()
            ))
            .into());
        }
    };

    let title = metadata_string(dictionary, "title", slug)?;
    let date = metadata_date(dictionary, "date", slug)?;
    let archived = metadata_bool(dictionary, "archived", slug)?;
    let tags = metadata_strings(dictionary, "tags", slug)?;

    assert!(!title.is_empty());
    assert!(!tags.is_empty());
    Ok(PostMetadata {
        title,
        date,
        archived,
        tags,
    })
}

fn metadata_string(dictionary: &Dict, key: &str, slug: &str) -> Result<String, Box<dyn Error>> {
    assert!(!key.is_empty());
    assert!(!slug.is_empty());

    let value = dictionary
        .get(key)
        .map_err(|error| io_error(format!("missing {key} metadata ({slug}): {error}")))?;
    let text = match value {
        Value::Str(text) => text.as_str(),
        value => {
            return Err(io_error(format!(
                "{key} metadata must be a string ({slug}), got {}",
                value.ty()
            ))
            .into());
        }
    };

    assert!(!text.is_empty());
    assert_eq!(text.trim(), text);
    Ok(text.to_owned())
}

fn metadata_bool(dictionary: &Dict, key: &str, slug: &str) -> Result<bool, Box<dyn Error>> {
    assert!(!key.is_empty());
    assert!(!slug.is_empty());

    let value = dictionary
        .get(key)
        .map_err(|error| io_error(format!("missing {key} metadata ({slug}): {error}")))?;
    let boolean = match value {
        Value::Bool(boolean) => *boolean,
        value => {
            return Err(io_error(format!(
                "{key} metadata must be a boolean ({slug}), got {}",
                value.ty()
            ))
            .into());
        }
    };

    assert!(matches!(value, Value::Bool(_)));
    if boolean {
        assert!(boolean);
    } else {
        assert!(!boolean);
    }
    Ok(boolean)
}

fn metadata_date(
    dictionary: &Dict,
    key: &str,
    slug: &str,
) -> Result<(i32, u8, u8), Box<dyn Error>> {
    assert!(!key.is_empty());
    assert!(!slug.is_empty());

    let value = dictionary
        .get(key)
        .map_err(|error| io_error(format!("missing {key} metadata ({slug}): {error}")))?;
    let datetime = match value {
        Value::Datetime(datetime) => datetime,
        value => {
            return Err(io_error(format!(
                "{key} metadata must be a datetime ({slug}), got {}",
                value.ty()
            ))
            .into());
        }
    };
    let year = datetime
        .year()
        .ok_or_else(|| io_error(format!("{key} metadata has no year ({slug})")))?;
    let month = datetime
        .month()
        .ok_or_else(|| io_error(format!("{key} metadata has no month ({slug})")))?;
    let day = datetime
        .day()
        .ok_or_else(|| io_error(format!("{key} metadata has no day ({slug})")))?;

    assert!(year >= 1970);
    assert!((1..=12).contains(&month));
    assert!((1..=31).contains(&day));
    Ok((year, month, day))
}

fn metadata_strings(
    dictionary: &Dict,
    key: &str,
    slug: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    assert!(!key.is_empty());
    assert!(!slug.is_empty());

    let value = dictionary
        .get(key)
        .map_err(|error| io_error(format!("missing {key} metadata ({slug}): {error}")))?;
    let values = match value {
        Value::Array(values) => values,
        value => {
            return Err(io_error(format!(
                "{key} metadata must be an array ({slug}), got {}",
                value.ty()
            ))
            .into());
        }
    };
    let strings = array_strings(values, key, slug)?;

    assert!(!strings.is_empty());
    assert!(strings.iter().all(|value| !value.is_empty()));
    Ok(strings)
}

fn array_strings(values: &Array, key: &str, slug: &str) -> Result<Vec<String>, Box<dyn Error>> {
    assert!(!key.is_empty());
    assert!(!slug.is_empty());

    let mut strings = Vec::with_capacity(values.len());
    for value in values {
        match value {
            Value::Str(text) => strings.push(text.as_str().to_owned()),
            value => {
                return Err(io_error(format!(
                    "{key} metadata entries must be strings ({slug}), got {}",
                    value.ty()
                ))
                .into());
            }
        }
    }

    assert_eq!(strings.len(), values.len());
    assert!(strings.iter().all(|value| value.trim() == value));
    Ok(strings)
}

fn extract_article_html(document: &str) -> Result<String, Box<dyn Error>> {
    assert!(document.starts_with("<!DOCTYPE html>"));
    assert!(document.contains("<body>"));

    let head = between(document, "<head>", "</head>")?;
    let body = between(document, "<body>", "</body>")?;
    let style = extract_optional_style(head)?;
    let mut article = String::with_capacity(style.len() + body.len());
    article.push_str(style);
    article.push_str(body);

    assert!(!article.contains("<script"));
    assert!(!article.contains("<body>"));
    Ok(article)
}

fn extract_optional_style(head: &str) -> Result<&str, Box<dyn Error>> {
    assert!(!head.contains("<body>"));
    assert!(!head.contains("</body>"));

    let Some(start) = head.find("<style>") else {
        return Ok("");
    };
    let relative_end = head[start..]
        .find("</style>")
        .ok_or_else(|| io_error("Typst emitted an unterminated style element"))?;
    let end = start + relative_end + "</style>".len();

    assert!(end <= head.len());
    assert!(!head[end..].contains("<style>"));
    Ok(&head[start..end])
}

fn between<'a>(input: &'a str, start: &str, end: &str) -> Result<&'a str, Box<dyn Error>> {
    assert!(!start.is_empty());
    assert!(!end.is_empty());

    let content_start = input
        .find(start)
        .map(|index| index + start.len())
        .ok_or_else(|| io_error(format!("missing {start} in Typst HTML output")))?;
    let relative_end = input[content_start..]
        .find(end)
        .ok_or_else(|| io_error(format!("missing {end} in Typst HTML output")))?;
    let content_end = content_start + relative_end;

    assert!(content_start <= content_end);
    assert!(content_end <= input.len());
    Ok(&input[content_start..content_end])
}

fn write_generated_posts(output_path: &Path, posts: &[CompiledPost]) -> Result<(), Box<dyn Error>> {
    assert!(!posts.is_empty());
    assert_eq!(
        output_path
            .extension()
            .and_then(|extension| extension.to_str()),
        Some("rs")
    );

    let mut generated = String::from("static GENERATED_POSTS: &[GeneratedPost] = &[\n");
    for post in posts {
        writeln!(generated, "    GeneratedPost {{")?;
        writeln!(generated, "        url: {:?},", post.slug)?;
        writeln!(generated, "        title: {:?},", post.title)?;
        writeln!(
            generated,
            "        date: ({}, {}, {}),",
            post.date.0, post.date.1, post.date.2
        )?;
        writeln!(generated, "        archived: {},", post.archived)?;
        write!(generated, "        tags: &[")?;
        for tag in &post.tags {
            write!(generated, "{:?},", tag)?;
        }
        writeln!(generated, "],")?;
        writeln!(generated, "        content: {:?},", post.content)?;
        writeln!(
            generated,
            "        estimated_read_time: {},",
            post.estimated_read_time
        )?;
        writeln!(generated, "    }},")?;
    }
    generated.push_str("];\n");

    assert!(generated.starts_with("static GENERATED_POSTS"));
    assert!(generated.ends_with("];\n"));
    fs::write(output_path, generated)?;
    Ok(())
}

fn emit_warnings(context: &str, warnings: &[SourceDiagnostic]) {
    assert!(!context.is_empty());
    assert!(warnings.len() <= u32::MAX as usize);

    for warning in warnings.iter().filter(|warning| {
        warning.message != "html export is under active development and incomplete"
    }) {
        println!("cargo:warning=Typst ({context}): {}", warning.message);
    }

    assert!(warnings.iter().all(|warning| !warning.message.is_empty()));
    assert!(warnings.len() <= u32::MAX as usize);
}

fn format_diagnostics(context: &str, diagnostics: &[SourceDiagnostic]) -> std::io::Error {
    assert!(!context.is_empty());
    assert!(!diagnostics.is_empty());

    let mut message = context.to_owned();
    for diagnostic in diagnostics {
        message.push_str("\n- ");
        message.push_str(&diagnostic.message);
        for hint in &diagnostic.hints {
            message.push_str("\n  hint: ");
            message.push_str(&hint.v);
        }
    }

    assert!(message.starts_with(context));
    assert!(message.len() > context.len());
    io_error(message)
}

fn io_error(message: impl Into<String>) -> std::io::Error {
    let message = message.into();
    assert!(!message.is_empty());
    assert!(message.len() < u32::MAX as usize);

    let error = std::io::Error::other(message);
    assert_eq!(error.kind(), std::io::ErrorKind::Other);
    assert!(!error.to_string().is_empty());
    error
}

struct CompiledPost {
    slug: String,
    title: String,
    date: (i32, u8, u8),
    archived: bool,
    tags: Vec<String>,
    content: String,
    estimated_read_time: usize,
}

struct PostMetadata {
    title: String,
    date: (i32, u8, u8),
    archived: bool,
    tags: Vec<String>,
}

struct SiteWorld {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
    main: FileId,
    source: Source,
    path: PathBuf,
}

impl SiteWorld {
    fn new(path: &str, text: String) -> Self {
        assert!(!path.is_empty());
        assert!(!text.is_empty());

        let fonts: Vec<Font> = typst_assets::fonts()
            .flat_map(|data| Font::iter(Bytes::new(data)))
            .collect();
        let book = FontBook::from_fonts(&fonts);
        let virtual_path = VirtualPath::new(path).expect("the source path is valid");
        let main = FileId::new(RootedPath::new(VirtualRoot::Project, virtual_path));
        let source = Source::new(main, text);

        assert!(!fonts.is_empty());
        assert!(book.contains_family("new computer modern math"));
        Self {
            library: LazyHash::new(
                Library::builder()
                    .with_features([Feature::Html].into_iter().collect())
                    .build(),
            ),
            book: LazyHash::new(book),
            fonts,
            main,
            source,
            path: PathBuf::from(path),
        }
    }
}

impl World for SiteWorld {
    fn library(&self) -> &LazyHash<Library> {
        assert!(!self.fonts.is_empty());
        assert_eq!(self.source.id(), self.main);
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        assert!(!self.fonts.is_empty());
        assert!(self.book.contains_family("new computer modern math"));
        &self.book
    }

    fn main(&self) -> FileId {
        assert!(!self.fonts.is_empty());
        assert_eq!(self.source.id(), self.main);
        self.main
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        assert!(!self.fonts.is_empty());
        assert_eq!(self.source.id(), self.main);
        if id == self.main {
            Ok(self.source.clone())
        } else {
            Err(FileError::NotFound(self.path.clone()))
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        assert!(!self.fonts.is_empty());
        assert_eq!(self.source.id(), self.main);
        let _ = id;
        Err(FileError::NotFound(self.path.clone()))
    }

    fn font(&self, index: usize) -> Option<Font> {
        assert!(!self.fonts.is_empty());
        assert!(self.book.contains_family("new computer modern math"));
        self.fonts.get(index).cloned()
    }

    fn today(&self, offset: Option<typst::foundations::Duration>) -> Option<Datetime> {
        assert!(!self.fonts.is_empty());
        assert_eq!(self.source.id(), self.main);
        let _ = offset;
        None
    }
}
