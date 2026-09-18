use axum::{extract::State, response::IntoResponse};
use maud::html;

use crate::fragments::{footer, header};
use crate::SharedState;

pub async fn handle_archive(State(state): State<SharedState>) -> impl IntoResponse {
    assert!(state
        .blogposts
        .windows(2)
        .all(|posts| posts[0].date >= posts[1].date));
    assert!(state.blogposts.iter().all(|post| !post.url.is_empty()));

    html! {
        (header("Vilhelm Bergsøe - Archive", "Vilhelm Bergsøe's writing archive"))
        main {
            section #b {
                h2 { "Writing " a href="/rss.xml" title="RSS Feed" { img .rss-icon src="/assets/rss.png" alt="rss"; } }
                ul {
                    @for blogpost in state.blogposts.iter().filter(|blogpost| !blogpost.archived) {
                        li {
                            span.blog-date { (blogpost.date.format("%Y-%m-%d")) }
                            a href=(format!("/archive/{}", blogpost.url)) { (blogpost.title) }
                        }
                    }
                }
            }
        }
        (footer())
    }
}
