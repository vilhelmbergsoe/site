use axum::{routing::get, Router};
use color_eyre::{eyre::eyre, eyre::Result, Report};
use comrak::plugins::syntect::SyntectAdapter;
use comrak::{markdown_to_html_with_plugins, Options, Plugins};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    combinator::{map, rest},
    multi::many0,
    sequence::delimited,
    IResult,
};

use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use tokio::time::Instant;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

use chrono::{offset::TimeZone, DateTime, NaiveDate, Utc};

use tower_http::{services::ServeDir, trace::TraceLayer};

pub mod handlers;
use handlers::{handle_404, handle_blog, handle_rss, root};

pub mod fragments;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "site=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // gets SITE_ROOT env var used for nix deployment
    let site_root = std::env::var("SITE_ROOT").unwrap_or_else(|_| "./".to_string());
    let path_prefix = Path::new(&site_root);

    tracing::info!("site root: {}", path_prefix.display());

    let state = new_state(path_prefix).await?;

    let app = Router::new()
        .route("/", get(root))
        .route("/blog/:url", get(handle_blog))
        .route("/rss.xml", get(handle_rss))
        .with_state(state)
        .nest_service(
            "/assets",
            ServeDir::new(path_prefix.join(Path::new("assets"))),
        )
        .fallback(get(handle_404));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.layer(TraceLayer::new_for_http()).into_make_service())
        .await?;

    Ok(())
}

#[derive(Clone)]
pub struct BlogPost {
    url: String,
    title: String,
    date: DateTime<Utc>,
    archived: bool,
    tags: Vec<String>,
    content: String,
    estimated_read_time: usize,
}

#[derive(Clone)]
pub struct AppState {
    blogposts: Vec<BlogPost>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Frontmatter {
    title: String,
    date: String,
    archived: bool,
    tags: Vec<String>,
}

fn parse_frontmatter(input: &str) -> IResult<&str, &str> {
    let delimiter = "---";

    let (input, frontmatter) =
        delimited(tag(delimiter), take_until(delimiter), tag(delimiter))(input)?;
    let content = input.trim_start();

    Ok((frontmatter, content))
}

#[derive(Debug)]
struct MathExpr {
    display_mode: bool,
    expr: String,
}

fn math_expr(input: &str) -> IResult<&str, MathExpr> {
    let (input, _) = tag("<span data-math-style=\"")(input)?;
    let (input, style) = take_until("\">")(input)?;
    let (input, _) = tag("\">")(input)?;
    let (input, expression) = take_until("</span>")(input)?;
    let (input, _) = tag("</span>")(input)?;
    Ok((
        input,
        MathExpr {
            display_mode: style == "display",
            expr: expression.to_string(),
        },
    ))
}

fn non_math_expr(input: &str) -> IResult<&str, String> {
    map(take_until("<span data-math-style=\""), |s: &str| {
        s.to_string()
    })(input)
}

fn parse_math_exprs(input: &str) -> IResult<&str, String> {
    let (input, parsed) = many0(alt((
        map(math_expr, |mathexpr| {
            let opts = katex::Opts::builder()
                .display_mode(mathexpr.display_mode)
                .output_type(katex::opts::OutputType::Mathml)
                .build()
                .unwrap();

            // Decode HTML entities for katex
            let decoded_expr = mathexpr.expr
                .replace("&gt;", ">")
                .replace("&lt;", "<")
                .replace("&amp;", "&");

            katex::render_with_opts(&decoded_expr, &opts).unwrap()
        }),
        non_math_expr,
    )))(input)?;

    let (input, remaining) = rest(input)?;

    Ok((input, format!("{}{}", parsed.concat(), remaining)))
}

async fn parse_blog(
    url: &str,
    path: &PathBuf,
    options: &Options<'_>,
    plugins: &Plugins<'_>,
) -> Result<BlogPost, Report> {
    let bytes = tokio::fs::read(path).await?;
    let text = String::from_utf8_lossy(&bytes);

    let Ok((frontmatter, content)) = parse_frontmatter(&text) else {
        return Err(eyre!(format!(
            "Error parsing frontmatter ({url}). Most likely missing delimiter \"---\\n\""
        )));
    };

    let frontmatter: Frontmatter = match serde_yaml::from_str(frontmatter) {
        Ok(fm) => fm,
        Err(err) => return Err(eyre!(format!("Error parsing blog ({url}): {err}"))),
    };

    let naive_date = NaiveDate::parse_from_str(&frontmatter.date, "%d-%m-%Y").unwrap();
    let naive_datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();
    let date: DateTime<Utc> = Utc.from_utc_datetime(&naive_datetime);

    let html = markdown_to_html_with_plugins(content, &options, &plugins);

    // Parse all math expressions
    let html = match parse_math_exprs(&html) {
        Ok((_, parsed)) => parsed,
        Err(err) => {
            return Err(eyre!(format!(
                "Error parsing math expressions for blog ({url}): {err}"
            )));
        }
    };

    Ok(BlogPost {
        url: url.to_string(),
        title: frontmatter.title,
        date,
        archived: frontmatter.archived,
        tags: frontmatter.tags,
        content: html,
        estimated_read_time: content.split_whitespace().count() / 200,
    })
}

async fn new_state(path_prefix: &Path) -> Result<AppState> {
    let mut blogposts: Vec<BlogPost> = Vec::new();

    let mut blog_dir = match tokio::fs::read_dir(path_prefix.join(Path::new("blog"))).await {
        Ok(dir) => dir,
        Err(err) => return Err(eyre!(format!("Error reading blog directory: {err}"))),
    };

    let adapter = SyntectAdapter::new(None);
    let mut options = Options::default();
    let mut plugins = Plugins::default();

    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.autolink = true;
    options.extension.footnotes = true;
    options.extension.header_ids = Some("".to_string());
    options.extension.math_dollars = true;
    options.render.unsafe_ = true;

    while let Some(entry) = blog_dir.next_entry().await? {
        let path = entry.path();
        if path.is_file() {
            // check for invalid file extensions
            let ext = path.extension();
            if ext != Some(std::ffi::OsStr::new("md"))
                && ext != Some(std::ffi::OsStr::new("markdown"))
                || ext.is_none()
            {
                tracing::warn!("skipping non markdown file: {}", path.display());
                continue;
            }

            if let Some(stem) = path.file_stem() {
                let url = stem.to_str().unwrap();

                // check if blogpost exists with same url
                if blogposts.par_iter().any(|b| b.url == url) {
                    tracing::warn!("skipping duplicate blogpost: {}", url);
                    continue;
                }

                plugins.render.codefence_syntax_highlighter = Some(&adapter);

                let start_time = Instant::now();
                let blogpost = parse_blog(url, &path, &options, &plugins).await?;
                let elapsed = start_time.elapsed().as_millis();

                blogposts.push(blogpost);
                tracing::info!("loaded blogpost - {} in {} ms", url, elapsed);
            }
        }
    }

    blogposts.sort_by(|a, b| b.date.cmp(&a.date));

    Ok(AppState { blogposts })
}

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
