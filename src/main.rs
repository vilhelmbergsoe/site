use std::{error::Error, net::SocketAddr};

use axum::{routing::get, Router};

pub mod handlers;
use handlers::{
    handle_404, handle_asset, handle_blog_index, handle_blog_post, handle_cv, handle_robots,
    handle_rss, handle_sitemap, handle_tag, root,
};

pub mod fragments;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    assert!(!GENERATED_POSTS.is_empty());
    assert!(GENERATED_POSTS
        .windows(2)
        .all(|posts| posts[0].date >= posts[1].date));

    let app = Router::new()
        .route("/", get(root))
        .route("/blog/", get(handle_blog_index))
        .route("/blog/:url", get(handle_blog_post))
        .route("/tag/:tag", get(handle_tag))
        .route("/cv.pdf", get(handle_cv))
        .route("/robots.txt", get(handle_robots))
        .route("/assets/*path", get(handle_asset))
        .route("/sitemap.xml", get(handle_sitemap))
        .route("/rss.xml", get(handle_rss))
        .fallback(get(handle_404));

    let address = SocketAddr::from(([0, 0, 0, 0], 8080));
    assert!(address.ip().is_unspecified());
    assert_eq!(address.port(), 8080);

    eprintln!("listening on {address}");
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

struct GeneratedPost {
    url: &'static str,
    title: &'static str,
    date: (i32, u8, u8),
    date_iso: &'static str,
    date_display: &'static str,
    date_rss: &'static str,
    archived: bool,
    tags: &'static [&'static str],
    content: &'static str,
    estimated_read_time: usize,
    // Whether the post contains live cells and needs /assets/live.js.
    interactive: bool,
}

include!(concat!(env!("OUT_DIR"), "/site/posts.rs"));
