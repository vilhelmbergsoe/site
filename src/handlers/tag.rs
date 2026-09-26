use axum::extract::Path;
use maud::{html, Markup};

use crate::fragments::{footer, header};
use crate::GENERATED_POSTS;

pub async fn handle_tag(Path(tag): Path<String>) -> Markup {
    let tagged_posts: Vec<_> = GENERATED_POSTS
        .iter()
        .filter(|post| !post.archived && post.tags.contains(&tag.as_str()))
        .collect();

    assert!(tagged_posts.iter().all(|post| !post.archived));
    assert!(tagged_posts
        .iter()
        .all(|post| post.tags.contains(&tag.as_str())));

    html! {
        (header(&format!("Vilhelm Bergsøe - Posts tagged with \"{}\"", tag), &format!("Vilhelm Bergsøe - Posts tagged with {}", tag)))
        main {
            section #b {
                h2 { "Posts tagged with: " (tag) }
                ul {
                    @for blogpost in &tagged_posts {
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
