use axum::{
    body,
    extract::State,
    http::{Response, StatusCode},
    response::IntoResponse,
};

use crate::templates;

use crate::AppState;

pub async fn handle_rss(State(state): State<AppState>) -> impl IntoResponse {
    let mut buf = Vec::new();

    templates::rss_feed_xml(&mut buf, state.blogposts).unwrap();

    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/rss+xml")
        .body(body::boxed(body::Full::from(buf)))
        .unwrap()
}
