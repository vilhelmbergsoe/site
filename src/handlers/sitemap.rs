use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
};
use crate::SharedState;

pub async fn handle_sitemap(State(state): State<SharedState>) -> impl IntoResponse {
    let mut sitemap = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);

    // Add static pages
    sitemap.push_str(r#"
    <url>
        <loc>https://bergsoe.net/</loc>
    </url>"#);

    // Add blog posts
    for post in &state.blogposts {
        if !post.archived {
            let url = format!(
                r#"
    <url>
        <loc>https://bergsoe.net/blog/{}</loc>
        <lastmod>{}</lastmod>
    </url>"#,
                post.url,
                post.date.format("%Y-%m-%d")
            );
            sitemap.push_str(&url);
        }
    }

    sitemap.push_str("</urlset>");

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/xml".parse().unwrap());

    (StatusCode::OK, headers, sitemap)
}
