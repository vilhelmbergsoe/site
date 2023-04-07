use axum::{
    extract::State,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use rayon::prelude::*;
use std::fmt;

use crate::AppState;

struct RssFeed<'a> {
    title: &'a str,
    link: &'a str,
    description: &'a str,
    items: Vec<RssItem<'a>>,
}

struct RssItem<'a> {
    guid: String,
    title: &'a str,
    link: String,
    description: String,
    pub_date: String,
}

impl fmt::Display for RssFeed<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><rss version=\"2.0\" xmlns:atom=\"http://www.w3.org/2005/Atom\"><channel><title>{}</title><link>{}</link><description>{}</description>",
            self.title, self.link, self.description
        )?;

        for item in &self.items {
            write!(f, "{item}")?;
        }

        write!(f, "</channel></rss>")
    }
}

impl fmt::Display for RssItem<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
        "<item><guid>{}</guid><title>{}</title><link>{}</link><description>{}</description><pubDate>{}</pubDate></item>",
        self.guid, self.title, self.link, self.description, self.pub_date)
    }
}

// TODO: possibly get rss feed at startup or use templating
// initial benchmarks seems as though this doesn't do much
pub async fn handle_rss(State(state): State<AppState>) -> impl IntoResponse {
    let items: Vec<RssItem> = state
        .blogposts
        .par_iter()
        .map(|post| RssItem {
            title: &post.title,
            link: format!("https://bergsoe.net/blog/{}", post.url),
            guid: format!("https://bergsoe.net/blog/{}", post.url),
            description: format!("tags: {}", post.tags.join(", ")),
            pub_date: post.date.to_rfc2822(),
        })
        .collect();

    // TODO: possibly use common config file in the future
    let feed = RssFeed {
        title: "Vilhelm's Blog",
        link: "https://bergsoe.net/",
        description: "My Blog RSS Feed",
        items,
    };

    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/rss+xml")
        .body(feed.to_string())
        .unwrap()
}
