use std::{error::Error, fs, path::Path};

use chrono::{Datelike as _, NaiveDate};
use typst::{
    foundations::{Array, Dict, Label, Str, Value},
    introspection::{Introspector, MetadataElem},
    model::Document,
};
use typst_html::HtmlDocument;

use crate::{
    compiler::{emit_diagnostics, emit_warnings, Compiler},
    io_error,
};

pub fn compile(compiler: &Compiler, source_path: &Path) -> Result<CompiledPost, Box<dyn Error>> {
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
    if !slug
        .bytes()
        .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
    {
        return Err(io_error(format!("post slug is not URL-safe: {slug}")).into());
    }

    let estimated_read_time = source.split_whitespace().count() / 200;
    let virtual_path = format!("blog/{slug}.typ");
    let world = compiler.world(&virtual_path, source);
    let compiled = typst::compile::<HtmlDocument>(&world);
    emit_warnings(&world, &compiled.warnings)?;
    let document = match compiled.output {
        Ok(document) => document,
        Err(diagnostics) => {
            emit_diagnostics(&world, &diagnostics)?;
            return Err(io_error(format!("Typst post compilation failed ({slug})")).into());
        }
    };

    let metadata = post_metadata(&document, &slug)?;
    let encoded = match typst_html::html(&document, &typst_html::HtmlOptions::default()) {
        Ok(encoded) => encoded,
        Err(diagnostics) => {
            emit_diagnostics(&world, &diagnostics)?;
            return Err(io_error(format!("Typst HTML export failed ({slug})")).into());
        }
    };
    let content = extract_article_html(&encoded)?;

    assert!(!metadata.title.is_empty());
    assert!(!content.is_empty());
    assert_eq!(
        document.info().title.as_deref(),
        Some(metadata.title.as_str())
    );
    assert_eq!(document.info().keywords.len(), metadata.tags.len());

    let date = NaiveDate::from_ymd_opt(
        metadata.date.0,
        u32::from(metadata.date.1),
        u32::from(metadata.date.2),
    )
    .ok_or_else(|| io_error(format!("invalid post date ({slug})")))?;
    let date_time = date
        .and_hms_opt(0, 0, 0)
        .ok_or_else(|| io_error(format!("invalid post time ({slug})")))?
        .and_utc();

    assert_eq!(date.year(), metadata.date.0);
    assert_eq!(date.month(), u32::from(metadata.date.1));
    Ok(CompiledPost {
        slug,
        title: metadata.title,
        date: metadata.date,
        date_iso: date.format("%Y-%m-%d").to_string(),
        date_display: date.format("%a %d %b %Y").to_string(),
        date_rss: date_time.to_rfc2822(),
        archived: metadata.archived,
        tags: metadata.tags,
        content,
        estimated_read_time,
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

pub struct CompiledPost {
    pub(crate) slug: String,
    pub(crate) title: String,
    pub(crate) date: (i32, u8, u8),
    pub(crate) date_iso: String,
    pub(crate) date_display: String,
    pub(crate) date_rss: String,
    pub(crate) archived: bool,
    pub(crate) tags: Vec<String>,
    pub(crate) content: String,
    pub(crate) estimated_read_time: usize,
}

struct PostMetadata {
    title: String,
    date: (i32, u8, u8),
    archived: bool,
    tags: Vec<String>,
}
