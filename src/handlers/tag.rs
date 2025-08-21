use axum::{
    extract::{Path, State},
    response::IntoResponse,
};
use maud::html;
use rayon::prelude::*;

use crate::fragments::{footer, header};
use crate::AppState;

pub async fn handle_tag(
    Path(tag): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let tagged_posts: Vec<_> = state
        .blogposts
        .par_iter()
        .filter(|p| !p.archived && p.tags.contains(&tag))
        .cloned()
        .collect();

    html! {
        (header(&format!("Vilhelm Bergsøe - Posts tagged with \"{}\"", tag), &format!("Vilhelm Bergsøe - Posts tagged with {}", tag)))
        main {
            section #b {
                h2 { "Posts tagged with: " (tag) }
                ul {
                    @for blogpost in &tagged_posts {
                        li {
                            (blogpost.date.format("D%d-%m-%Y "))
                            a href=(format!("/blog/{}", blogpost.url)) { (blogpost.title) }
                        }
                    }
                }
            }
        }
        (footer())
    }
}
