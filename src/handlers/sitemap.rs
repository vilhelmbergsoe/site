use std::fmt::Write as _;

use axum::{http::header, response::IntoResponse};

use crate::GENERATED_POSTS;

pub async fn handle_sitemap() -> impl IntoResponse {
    assert!(!GENERATED_POSTS.is_empty());
    assert!(GENERATED_POSTS.iter().all(|post| !post.url.is_empty()));

    let mut sitemap = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n\
    <url>\n        <loc>https://bergsoe.net/</loc>\n    </url>\n\
    <url>\n        <loc>https://bergsoe.net/archive/</loc>\n    </url>\n",
    );

    for post in GENERATED_POSTS.iter().filter(|post| !post.archived) {
        writeln!(
            sitemap,
            "    <url>\n        <loc>https://bergsoe.net/archive/{}</loc>\n        <lastmod>{}</lastmod>\n    </url>",
            post.url, post.date_iso,
        )
        .expect("writing to a String cannot fail");
    }
    sitemap.push_str("</urlset>\n");

    assert!(sitemap.starts_with("<?xml"));
    assert!(sitemap.ends_with("</urlset>\n"));
    ([(header::CONTENT_TYPE, "application/xml")], sitemap)
}
