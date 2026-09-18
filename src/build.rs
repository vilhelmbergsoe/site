use std::{env, error::Error, fs, path::PathBuf};

use ructe::Ructe;
use typst::{
    diag::{FileError, FileResult, SourceDiagnostic},
    foundations::{Bytes, Datetime, Duration},
    syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot},
    text::{Font, FontBook},
    utils::LazyHash,
    Library, LibraryExt, World,
};
use typst_layout::PagedDocument;

fn main() -> Result<(), Box<dyn Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let cv_source = manifest_dir.join("cv/cv.typ");
    let cv_pdf = PathBuf::from(env::var("OUT_DIR")?).join("cv.pdf");

    assert!(manifest_dir.is_absolute());
    assert!(cv_source.starts_with(&manifest_dir));

    println!("cargo:rerun-if-changed={}", cv_source.display());
    Ructe::from_env()?.compile_templates("templates")?;
    compile_cv(&cv_source, &cv_pdf)?;

    assert!(cv_pdf.is_file());
    assert!(fs::metadata(cv_pdf)?.len() > 1_024);
    Ok(())
}

fn compile_cv(source_path: &PathBuf, output_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    assert!(source_path.is_file());
    assert_eq!(
        source_path
            .extension()
            .and_then(|extension| extension.to_str()),
        Some("typ")
    );

    let source = fs::read_to_string(source_path)?;
    let world = CvWorld::new(source);
    let compiled = typst::compile::<PagedDocument>(&world);

    for warning in &compiled.warnings {
        println!("cargo:warning=Typst: {}", warning.message);
    }

    let document = compiled
        .output
        .map_err(|diagnostics| format_diagnostics("Typst compilation failed", &diagnostics))?;
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
    std::io::Error::other(message)
}

struct CvWorld {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
    main: FileId,
    source: Source,
}

impl CvWorld {
    fn new(text: String) -> Self {
        assert!(!text.is_empty());
        assert!(text.contains("Vilhelm Bergsøe"));

        let fonts: Vec<Font> = typst_assets::fonts()
            .flat_map(|data| Font::iter(Bytes::new(data)))
            .collect();
        let book = FontBook::from_fonts(&fonts);
        let main = FileId::new(RootedPath::new(
            VirtualRoot::Project,
            VirtualPath::new("cv.typ").expect("the CV path is valid"),
        ));
        let source = Source::new(main, text);

        assert!(!fonts.is_empty());
        assert!(
            book.contains_family("libertinus serif"),
            "embedded font families: {:?}",
            fonts
                .iter()
                .map(|font| &font.info().family)
                .collect::<Vec<_>>()
        );
        Self {
            library: LazyHash::new(Library::builder().build()),
            book: LazyHash::new(book),
            fonts,
            main,
            source,
        }
    }
}

impl World for CvWorld {
    fn library(&self) -> &LazyHash<Library> {
        assert!(!self.fonts.is_empty());
        assert_eq!(self.source.id(), self.main);
        &self.library
    }

    fn book(&self) -> &LazyHash<FontBook> {
        assert!(!self.fonts.is_empty());
        assert!(self.book.contains_family("libertinus serif"));
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
            Err(FileError::NotFound(PathBuf::from("cv.typ")))
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        assert!(!self.fonts.is_empty());
        assert_eq!(self.source.id(), self.main);
        let _ = id;
        Err(FileError::NotFound(PathBuf::from("cv.typ")))
    }

    fn font(&self, index: usize) -> Option<Font> {
        assert!(!self.fonts.is_empty());
        assert!(self.book.contains_family("libertinus serif"));
        self.fonts.get(index).cloned()
    }

    fn today(&self, offset: Option<Duration>) -> Option<Datetime> {
        assert!(!self.fonts.is_empty());
        assert_eq!(self.source.id(), self.main);
        let _ = offset;
        None
    }
}
