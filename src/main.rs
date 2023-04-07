use axum::{routing::get, Router};
use color_eyre::{eyre::eyre, eyre::Result, Report};
use comrak::plugins::syntect::SyntectAdapter;
use comrak::{markdown_to_html_with_plugins, ComrakOptions, ComrakPlugins};
use nom::{
    bytes::complete::{tag, take_until},
    sequence::delimited,
    IResult,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
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

    let state = new_state(path_prefix).unwrap();

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
        .await
        .unwrap();

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

fn parse_blog(
    url: &str,
    path: &PathBuf,
    options: &ComrakOptions,
    plugins: &ComrakPlugins,
) -> Result<BlogPost, Report> {
    let text = std::fs::read_to_string(path).unwrap();

    let Ok((frontmatter, content)) = parse_frontmatter(&text) else {
        return Err(eyre!(format!(
            "Error parsing frontmatter ({url}). Most likely missing delimiter \"---\\n\""
        )))
    };

    let frontmatter: Frontmatter = match serde_yaml::from_str(frontmatter) {
        Ok(fm) => fm,
        Err(err) => return Err(eyre!(format!("Error parsing blog ({url}): {err}"))),
    };

    let naive_date = NaiveDate::parse_from_str(&frontmatter.date, "%d-%m-%Y").unwrap();
    let naive_datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();
    let date: DateTime<Utc> = Utc.from_utc_datetime(&naive_datetime);

    let html = markdown_to_html_with_plugins(content, &options, &plugins);

    Ok(BlogPost {
        url: url.to_string(),
        title: frontmatter.title,
        date,
        archived: frontmatter.archived,
        tags: frontmatter.tags,
        content: html,
    })
}

fn new_state(path_prefix: &Path) -> Result<AppState> {
    // Blog posts
    let mut blogposts: Vec<BlogPost> = Vec::new();

    let blog_dir = match std::fs::read_dir(path_prefix.join(Path::new("blog"))) {
        Ok(dir) => dir,
        Err(err) => return Err(eyre!(format!("Error reading blog directory: {err}"))),
    };

    for entry in blog_dir {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(stem) = path.file_stem() {
                let url = stem.to_str().unwrap();

                // TODO: implement own theme
                let adapter = SyntectAdapter::new("base16-eighties.dark");
                let options = ComrakOptions::default();
                let mut plugins = ComrakPlugins::default();

                plugins.render.codefence_syntax_highlighter = Some(&adapter);

                let blogpost = parse_blog(url, &path, &options, &plugins)?;
                blogposts.push(blogpost);
                tracing::info!("loaded blogpost - {}", url);
            }
        }
    }

    blogposts.sort_by(|a, b| b.date.cmp(&a.date));

    Ok(AppState { blogposts })
}
