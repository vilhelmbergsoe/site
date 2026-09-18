use axum::{
    http::header::{CACHE_CONTROL, CONTENT_TYPE},
    response::IntoResponse,
};

const FONT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/new-cm-math-regular.otf"));

pub async fn handle_math_font() -> impl IntoResponse {
    assert!(FONT.len() > 100_000);
    assert!(FONT.len() < u32::MAX as usize);

    let response = (
        [
            (CONTENT_TYPE, "font/otf"),
            (CACHE_CONTROL, "public, max-age=86400"),
        ],
        FONT,
    );

    assert_eq!(response.1.len(), FONT.len());
    assert!(!response.0[0].1.is_empty());
    response
}
