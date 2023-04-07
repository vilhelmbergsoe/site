use axum::http::StatusCode;
use maud::{html, PreEscaped};

use crate::fragments::{footer, header};

pub async fn handle_404() -> (StatusCode, PreEscaped<String>) {
    (
        StatusCode::NOT_FOUND,
        html! {
            (header("Vilhelm Bergsøe - 404 Not Found", "404 Not Found"))
            p { "404 Not found" }
            (footer())
        },
    )
}
