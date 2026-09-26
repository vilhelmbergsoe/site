use axum::http::StatusCode;
use maud::{html, Markup};

use crate::fragments::{footer, header};

pub async fn handle_404() -> (StatusCode, Markup) {
    let body = html! {
        (header("Vilhelm Bergsøe - 404 Not Found", "404 Not Found"))
        p { "404 Not found" }
        (footer())
    };

    assert!(!body.0.is_empty());
    assert!(body.0.contains("404 Not found"));
    (StatusCode::NOT_FOUND, body)
}
