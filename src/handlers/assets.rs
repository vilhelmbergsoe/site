use axum::{
    extract::Path,
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};

struct Asset {
    path: &'static str,
    content_type: &'static str,
    data: &'static [u8],
}

macro_rules! source_asset {
    ($path:literal, $content_type:literal) => {
        Asset {
            path: $path,
            content_type: $content_type,
            data: include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/", $path)),
        }
    };
}

// Keeping this manifest explicit makes every public file visible to the compiler
// and avoids a procedural macro, directory copy, and runtime MIME detection.
static ASSETS: &[Asset] = &[
    source_asset!("bergsoe.webp", "image/webp"),
    Asset {
        path: "cv.pdf",
        content_type: "application/pdf",
        data: include_bytes!(concat!(env!("OUT_DIR"), "/site/cv.pdf")),
    },
    source_asset!("favicon.svg", "image/svg+xml"),
    Asset {
        path: "fonts/new-cm-math-regular.otf",
        content_type: "font/otf",
        data: include_bytes!(concat!(
            env!("OUT_DIR"),
            "/site/fonts/new-cm-math-regular.otf"
        )),
    },
    source_asset!("fonts/source-serif-4-italic.woff2", "font/woff2"),
    source_asset!("fonts/source-serif-4.woff2", "font/woff2"),
    source_asset!("gpg.txt", "text/plain; charset=utf-8"),
    source_asset!("pictures/ascii-guitar.webp", "image/webp"),
    source_asset!("pictures/neural_nets/multi_neuron.webp", "image/webp"),
    source_asset!("pictures/neural_nets/neural_network.webp", "image/webp"),
    source_asset!("pictures/neural_nets/nn_cost.webp", "image/webp"),
    source_asset!("pictures/neural_nets/perceptron.webp", "image/webp"),
    source_asset!("robots.txt", "text/plain; charset=utf-8"),
    source_asset!("rss.png", "image/png"),
    source_asset!("style.css", "text/css; charset=utf-8"),
];

pub async fn handle_asset(Path(path): Path<String>) -> Response {
    let response = serve_asset(&path);
    assert!(!response.status().is_server_error());
    assert_ne!(response.status(), StatusCode::MOVED_PERMANENTLY);
    response
}

pub async fn handle_cv() -> Response {
    let mut response = serve_asset("cv.pdf");
    if response.status() == StatusCode::OK {
        response.headers_mut().insert(
            header::CONTENT_DISPOSITION,
            HeaderValue::from_static("inline; filename=cv.pdf"),
        );
    }

    assert_eq!(response.status(), StatusCode::OK);
    assert!(response.headers().get(header::CONTENT_TYPE).is_some());
    response
}

pub async fn handle_robots() -> Response {
    let response = serve_asset("robots.txt");
    assert_eq!(response.status(), StatusCode::OK);
    assert!(response.headers().get(header::CONTENT_TYPE).is_some());
    response
}

fn serve_asset(path: &str) -> Response {
    if path.starts_with('/') {
        return not_found();
    }
    if path.contains('\\') {
        return not_found();
    }

    let Some(asset) = ASSETS.iter().find(|asset| asset.path == path) else {
        return not_found();
    };

    assert!(!asset.content_type.is_empty());
    assert!(!asset.data.is_empty());
    (
        [
            (
                header::CONTENT_TYPE,
                HeaderValue::from_static(asset.content_type),
            ),
            (
                header::CACHE_CONTROL,
                HeaderValue::from_static("public, max-age=86400"),
            ),
        ],
        asset.data,
    )
        .into_response()
}

fn not_found() -> Response {
    let response = StatusCode::NOT_FOUND.into_response();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    assert!(!response.status().is_success());
    response
}

#[cfg(test)]
mod tests {
    use super::ASSETS;

    #[test]
    fn asset_manifest_is_sorted_and_unique() {
        assert!(ASSETS.windows(2).all(|pair| pair[0].path < pair[1].path));
        assert!(ASSETS.iter().all(|asset| !asset.data.is_empty()));
    }
}
