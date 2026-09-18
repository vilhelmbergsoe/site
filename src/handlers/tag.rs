use crate::fragments::{footer, header};
use crate::SharedState;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use maud::html;

pub async fn handle_tag(
    Path(tag): Path<String>,
    State(state): State<SharedState>,
) -> impl IntoResponse {
    let tagged_posts: Vec<_> = state
        .blogposts
        .iter()
        .filter(|p| !p.archived && p.tags.contains(&tag))
        .collect();

    html! {
        (header(&format!("Vilhelm Bergsøe - Posts tagged with \"{}\"", tag), &format!("Vilhelm Bergsøe - Posts tagged with {}", tag)))
        main {
            section #b {
                h2 { "Posts tagged with: " (tag) }
                ul {
                    @for blogpost in &tagged_posts {
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
