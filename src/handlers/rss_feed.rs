use std::fmt::Write as _;

use axum::{http::header, response::IntoResponse};

use crate::{GeneratedPost, GENERATED_POSTS};

pub async fn handle_rss() -> impl IntoResponse {
    let feed = render_rss(GENERATED_POSTS);
    assert!(feed.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\" ?>"));
    assert!(feed.ends_with("</rss>\n"));

    ([(header::CONTENT_TYPE, "application/rss+xml")], feed)
}

fn render_rss(posts: &[GeneratedPost]) -> String {
    assert!(!posts.is_empty());
    assert!(posts.iter().all(|post| !post.url.is_empty()));

    let mut feed = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\" ?>\n\
<rss version=\"2.0\" xmlns:atom=\"http://www.w3.org/2005/Atom\">\n\
    <channel>\n\
        <title>Vilhelm Bergsøe&apos;s Blog</title>\n\
        <link>https://bergsoe.net/blog/</link>\n\
        <description>Vilhelm Bergsøe&apos;s blog feed</description>\n",
    );

    for post in posts.iter().filter(|post| !post.archived) {
        let url = format!("https://bergsoe.net/blog/{}", post.url);
        let tags = post.tags.join(", ");
        writeln!(
            feed,
            "        <item>\n            <guid>{}</guid>\n            <title>{}</title>\n            <link>{}</link>\n            <description>tags: {}</description>\n            <pubDate>{}</pubDate>\n        </item>",
            escape_xml(&url),
            escape_xml(post.title),
            escape_xml(&url),
            escape_xml(&tags),
            escape_xml(post.date_rss),
        )
        .expect("writing to a String cannot fail");
    }
    feed.push_str("    </channel>\n</rss>\n");

    assert!(feed.contains("<channel>"));
    assert!(feed.contains("</channel>"));
    feed
}

fn escape_xml(value: &str) -> String {
    assert!(value.len() < u32::MAX as usize);
    assert!(!value.contains('\0'));

    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '\'' => escaped.push_str("&apos;"),
            '"' => escaped.push_str("&quot;"),
            character => escaped.push(character),
        }
    }

    assert!(escaped.len() >= value.len());
    assert!(!escaped.contains('\0'));
    escaped
}

#[cfg(test)]
mod tests {
    use super::escape_xml;

    #[test]
    fn escapes_xml_metacharacters() {
        assert_eq!(escape_xml("<&>'\""), "&lt;&amp;&gt;&apos;&quot;");
        assert_eq!(escape_xml("Bergsøe"), "Bergsøe");
    }
}
