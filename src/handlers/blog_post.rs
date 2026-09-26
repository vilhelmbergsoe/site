use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use maud::{html, PreEscaped};

use crate::fragments::{footer, header};
use crate::{handle_404, GENERATED_POSTS};

pub async fn handle_blog_post(Path(url): Path<String>) -> Response {
    let blogpost = GENERATED_POSTS.iter().find(|blogpost| blogpost.url == url);
    assert!(GENERATED_POSTS.iter().all(|post| !post.content.is_empty()));
    assert!(GENERATED_POSTS.iter().all(|post| !post.title.is_empty()));

    let Some(blogpost) = blogpost else {
        return handle_404().await.into_response();
    };

    (
        StatusCode::OK,
        html! {
            (header(&format!("Vilhelm Bergsøe - {}", blogpost.title), "Vilhelm Bergsøe's blog"))
            main {
                section #h {
                    div .blogpost {
                        h2 .blogtitle { (blogpost.title) }
                        span style="opacity: 0.7;" {
                            (blogpost.date_display)
                            (format!(" - {} min read", blogpost.estimated_read_time))
                        }
                        br;
                        (PreEscaped(blogpost.content))
                    }

                    div {
                        "tags: ["
                        @for (index, tag) in blogpost.tags.iter().enumerate() {
                            @if index > 0 {
                                ", "
                            }
                            a href=(format!("/tag/{tag}")) { (tag) }
                        }
                        "]"
                    }
                }
            }
            (footer())
        },
    )
        .into_response()
}
