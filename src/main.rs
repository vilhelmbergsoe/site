use axum::{routing::get, Router};
use color_eyre::{eyre::Result, Report};
use comrak::{markdown_to_html, ComrakOptions};
use nom::{
    bytes::complete::{tag, take_until},
    sequence::delimited,
    IResult,
};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::{collections::HashMap, net::SocketAddr};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

use chrono::{offset::TimeZone, DateTime, NaiveDate, Utc};

use tower_http::{services::ServeDir, trace::TraceLayer};

mod handlers;
use handlers::{handle_404, handle_blog, root};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "site=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = new_state().unwrap();

    let path = std::env::current_dir().unwrap();
    tracing::debug!("current working directory: {}", path.display());

    let app = Router::new()
        .route("/", get(root))
        .with_state(state.clone())
        .route("/blog/:url", get(handle_blog).with_state(state))
        .nest_service("/assets", ServeDir::new("assets"))
        .fallback(get(handle_404));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.layer(TraceLayer::new_for_http()).into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[derive(Clone)]
struct BlogPost {
    url: String,
    title: String,
    date: DateTime<Utc>,
    archived: bool,
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
    path: &std::path::PathBuf,
    options: ComrakOptions,
) -> Result<BlogPost, Report> {
    let text = std::fs::read_to_string(&path).unwrap();

    let (frontmatter, content) = parse_frontmatter(&text).unwrap();
    let frontmatter: Frontmatter = serde_yaml::from_str(frontmatter)?;

    let naive_date = NaiveDate::parse_from_str(&frontmatter.date, "%d-%m-%Y").unwrap();
    let naive_datetime = naive_date.and_hms_opt(0, 0, 0).unwrap();
    let date: DateTime<Utc> = Utc.from_utc_datetime(&naive_datetime);

    let html = markdown_to_html(content, &options);

    Ok(BlogPost {
        url: url.to_string(),
        title: frontmatter.title,
        date,
        archived: frontmatter.archived,
        content: html,
    })
}

fn new_state() -> Result<AppState> {
    let mut state = AppState {
        blogposts: Vec::new(),
    };

    let mut blogposts: Vec<BlogPost> = Vec::new();

    for entry in std::fs::read_dir("blog")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(stem) = path.file_stem() {
                let url = stem.to_str().unwrap();

                // TODO: add syntax highlighting for code blocks
                let options = ComrakOptions::default();

                let blogpost = parse_blog(url, &path, options)?;
                blogposts.push(blogpost);
                tracing::debug!("loaded blogpost - {}", url);
            }
        }
    }

    blogposts.sort_by(|a, b| b.date.cmp(&a.date));

    state.blogposts = blogposts;

    Ok(state)
}
