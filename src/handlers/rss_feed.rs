use axum::{
    body,
    extract::State,
    http::{Response, StatusCode},
    response::IntoResponse,
};

use crate::{templates, SharedState};

pub async fn handle_rss(State(state): State<SharedState>) -> impl IntoResponse {
    let mut buf = Vec::new();

    let published_posts = state
        .blogposts
        .iter()
        .filter(|post| !post.archived)
        .cloned()
        .collect();
    templates::rss_feed_xml(&mut buf, published_posts).unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/rss+xml")
        .body(body::boxed(body::Full::from(buf)))
        .unwrap()
}
