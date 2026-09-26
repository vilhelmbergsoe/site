use maud::{html, Markup};

use crate::fragments::{footer, header};
use crate::GENERATED_POSTS;

pub async fn handle_archive() -> Markup {
    assert!(!GENERATED_POSTS.is_empty());
    assert!(GENERATED_POSTS
        .windows(2)
        .all(|posts| posts[0].date >= posts[1].date));

    html! {
        (header("Vilhelm Bergsøe - Archive", "Vilhelm Bergsøe's writing archive"))
        main {
            section #b {
                h2 { "Writing " a href="/rss.xml" title="RSS Feed" { img .rss-icon src="/assets/rss.png" alt="rss"; } }
                ul {
                    @for blogpost in GENERATED_POSTS.iter().filter(|blogpost| !blogpost.archived) {
                        li {
                            span.blog-date { (blogpost.date_iso) }
                            a href=(format!("/archive/{}", blogpost.url)) { (blogpost.title) }
                        }
                    }
                }
            }
        }
        (footer())
    }
}
