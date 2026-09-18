use std::{
    hash::{DefaultHasher, Hash, Hasher},
    net::SocketAddr,
};

use axum::{
    extract::{ConnectInfo, Path, State},
    http::{HeaderValue, StatusCode},
    response::{IntoResponse, Redirect},
};
use maud::{html, PreEscaped};

use crate::{
    fragments::{footer, header},
    handle_404, SharedState, UserId,
};

pub async fn redirect_legacy_blog(Path(url): Path<String>) -> Result<Redirect, StatusCode> {
    assert!(!url.is_empty());
    assert!(!url.starts_with('/'));

    let destination = format!("/archive/{url}");
    if HeaderValue::try_from(destination.as_str()).is_err() {
        return Err(StatusCode::BAD_REQUEST);
    }

    assert!(destination.starts_with("/archive/"));
    assert!(destination.len() > "/archive/".len());
    Ok(Redirect::permanent(&destination))
}

pub async fn handle_blog(
    Path(url): Path<String>,
    State(state): State<SharedState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let blogpost = state.blogposts.iter().find(|blogpost| blogpost.url == url);

    if let Some(blogpost) = &blogpost {
        let mut hasher = DefaultHasher::new();
        state.salt.hash(&mut hasher);
        addr.ip().hash(&mut hasher);
        let user_id: UserId = hasher.finish();

        let mut write_guard = state.total_views.write().await;
        write_guard
            .entry(blogpost.title.clone())
            .or_default()
            .insert(user_id);
    }

    match blogpost {
        Some(blogpost) => {
            let read_guard = state.total_views.read().await;
            let total_views = read_guard
                .get(&blogpost.title)
                .map_or(0, |views_set| views_set.len());
            (
                StatusCode::OK,
                html! {
                    (header(&format!("Vilhelm Bergsøe - {}", blogpost.title), "Vilhelm Bergsøe - Writing"))
                    main {
                        section #h {
                            div .blogpost {
                                h2 .blogtitle { (blogpost.title) }
                                span style="opacity: 0.7;" {
                                    (blogpost.date.format("%a %d %b %Y"))
                                    // 200 words per minute estimate
                                    (format!(" - {} min read | {} view(s)" , blogpost.estimated_read_time, total_views))
                                }
                                br;
                                (PreEscaped(&blogpost.content))
                            }

                            div {
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
            )
        }
        None => handle_404().await,
    }
}
