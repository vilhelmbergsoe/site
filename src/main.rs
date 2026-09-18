use axum::{routing::get, Router};
use color_eyre::eyre::Result;
use rand::prelude::*;
use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

use chrono::{offset::TimeZone, DateTime, Utc};

use tower_http::{services::ServeDir, services::ServeFile, trace::TraceLayer};

pub mod handlers;
use handlers::{
    handle_404, handle_archive, handle_blog, handle_cv, handle_math_font, handle_rss,
    handle_sitemap, handle_stats, handle_tag, redirect_legacy_blog, root,
};

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

    let state = new_state();

    let app = Router::new()
        .route("/", get(root))
        .route("/archive/", get(handle_archive))
        .route("/archive/:url", get(handle_blog))
        .route("/blog/:url", get(redirect_legacy_blog))
        .route("/tag/:tag", get(handle_tag))
        .route("/stats", get(handle_stats))
        .route("/cv.pdf", get(handle_cv))
        .route(
            "/assets/fonts/new-cm-math-regular.otf",
            get(handle_math_font),
        )
        .route("/sitemap.xml", get(handle_sitemap))
        .route("/rss.xml", get(handle_rss))
        .route_service(
            "/robots.txt",
            ServeFile::new(path_prefix.join(Path::new("assets/robots.txt"))),
        )
        .with_state(state.into())
        .nest_service(
            "/assets",
            ServeDir::new(path_prefix.join(Path::new("assets"))),
        )
        .fallback(get(handle_404));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(
            app.layer(TraceLayer::new_for_http())
                .into_make_service_with_connect_info::<SocketAddr>(),
        )
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

pub type UserId = u64;

pub struct State {
    blogposts: Vec<BlogPost>,
    uptime: DateTime<Utc>,
    total_views: RwLock<HashMap<String, HashSet<UserId>>>,
    salt: u64,
}

pub type SharedState = Arc<State>;

struct GeneratedPost {
    url: &'static str,
    title: &'static str,
    date: (i32, u8, u8),
    archived: bool,
    tags: &'static [&'static str],
    content: &'static str,
    estimated_read_time: usize,
}

include!(concat!(env!("OUT_DIR"), "/posts.rs"));

fn new_state() -> SharedState {
    assert!(!GENERATED_POSTS.is_empty());
    assert!(GENERATED_POSTS
        .windows(2)
        .all(|posts| posts[0].date >= posts[1].date));

    let blogposts = GENERATED_POSTS
        .iter()
        .map(|post| {
            let date = Utc
                .with_ymd_and_hms(
                    post.date.0,
                    u32::from(post.date.1),
                    u32::from(post.date.2),
                    0,
                    0,
                    0,
                )
                .single()
                .expect("build-time validated post date");
            BlogPost {
                url: post.url.to_owned(),
                title: post.title.to_owned(),
                date,
                archived: post.archived,
                tags: post.tags.iter().map(|tag| (*tag).to_owned()).collect(),
                content: post.content.to_owned(),
                estimated_read_time: post.estimated_read_time,
            }
        })
        .collect::<Vec<_>>();

    assert_eq!(blogposts.len(), GENERATED_POSTS.len());
    assert!(blogposts.iter().all(|post| !post.content.is_empty()));

    let salt = rand::rng().random::<u64>();
    tracing::info!("Generated server salt for this session");

    Arc::new(State {
        blogposts,
        uptime: chrono::Utc::now(),
        total_views: RwLock::new(HashMap::new()),
        salt,
    })
}

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
