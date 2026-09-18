use axum::{
    http::header::{CONTENT_DISPOSITION, CONTENT_TYPE},
    response::IntoResponse,
};

const CV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/cv.pdf"));

pub async fn handle_cv() -> impl IntoResponse {
    assert!(CV.starts_with(b"%PDF-"));
    assert!(CV.len() > 1_024);

    (
        [
            (CONTENT_TYPE, "application/pdf"),
            (
                CONTENT_DISPOSITION,
                "inline; filename=cv.pdf",
            ),
        ],
        CV,
    )
}
