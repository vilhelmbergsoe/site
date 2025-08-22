use axum::{
    body,
    extract::State,
    http::{Response, StatusCode},
    response::IntoResponse,
};

use crate::{templates, SharedState};

pub async fn handle_rss(State(state): State<SharedState>) -> impl IntoResponse {
    let mut buf = Vec::new();

    templates::rss_feed_xml(&mut buf, state.blogposts.clone()).unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/rss+xml")
        .body(body::boxed(body::Full::from(buf)))
        .unwrap()
}
