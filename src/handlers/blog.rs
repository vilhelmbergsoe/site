use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use maud::{html, PreEscaped};

use rayon::prelude::*;

use crate::fragments::{footer, header};
use crate::handle_404;

use crate::AppState;

pub async fn handle_blog(
    Path(url): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let blogpost = state
        .blogposts
        .par_iter()
        .find_first(|blogpost| blogpost.url == url);

    match blogpost {
        Some(blogpost) => (
            StatusCode::OK,
            html! {
                (header(&format!("Vilhelm Bergsøe - {}", blogpost.title), "Vilhelm Bergsøe - Blog"))
                main {
                    section #h {
                        div .blogpost {
                            h2 .blogtitle { (blogpost.title) }
                            span style="opacity: 0.7;" {
                                (blogpost.date.format("%a %d %b %Y"))
                                // 200 words per minute estimate
                                (format!(" - {} min read", blogpost.estimated_read_time))
                            }
                            br;
                            p {
                                (PreEscaped(&blogpost.content))
                            }
                        }

                        div style="opacity: 0.7;" {
                            "tags: ["
                            @for (i, tag) in blogpost.tags.iter().enumerate() {
                                @if i > 0 {
                                    ", "
                                }
                                a href=(format!("/tag/{}", tag)) { (tag) }
                            }
                            "]"
                        }
                    }
                }

                (footer())
            },
        ),
        None => handle_404().await,
    }
}
