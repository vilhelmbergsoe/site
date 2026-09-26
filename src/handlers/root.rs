use maud::{html, Markup};

use crate::fragments::{footer, header};
use crate::GENERATED_POSTS;

pub async fn root() -> Markup {
    assert!(!GENERATED_POSTS.is_empty());
    assert!(GENERATED_POSTS.iter().all(|post| !post.url.is_empty()));

    html! {
        (header("Vilhelm Bergsøe - Home", "Vilhelm Bergsøe's personal website and blog"))
        div style="position: absolute; left: -9999px; top: -9999px; width: 1px; height: 1px; overflow: hidden;" {
            a href="/babble/blog" { "My blog" }
            a href="/babble/wp-login" { "Wordpress Login" }
            a href="/babble/btc-wallet" { "Bitcoin Wallet" }
        }

        main {
            section { h2 { "Info" } p { "Software Developer and
                mathematics student from Copenhagen. I'm interested in systems programming,
                mathematics, economics and whatever else catches my
                attention." }

                ul .profile-links {
                    li {
                        a href="mailto:vilhelm@bergsoe.net" { "Email" }
                        " | "
                        a href="/assets/gpg.txt" { "GPG key" }
                    }
                    li {
                        a href="/cv.pdf" { "CV" }
                    }
                    li {
                        span { a href="https://tangled.org/bergsoe.net" { "Tangled" } " | " a href="https://codeberg.org/vilhelmbergsoe" { "Codeberg" } " | " a href="https://github.com/vilhelmbergsoe" { "GitHub" } }
                    }
                }

                h3 { "Projects" }
                ul {
                    li { a href="https://tangled.org/bergsoe.net/thread" { "thread" } " - My (very in-progress) native code debugger." }
                    li { a href="https://tangled.org/bergsoe.net/nod" { "nod" } " - Nix observability daemon for monitoring builds and substitutions." }
                    li { a href="https://github.com/vilhelmbergsoe/brainybishop" { "brainybishop" } " - Simple little chess engine." }
                }

                h3 { "Latest posts" }
                ul .post-list {
                    @for blogpost in GENERATED_POSTS.iter().filter(|blogpost| !blogpost.archived).take(3) {
                        li {
                            span.blog-date { (blogpost.date.0) }
                            a href=(format!("/blog/{}", blogpost.url)) { (blogpost.title) }
                        }
                    }
                }
                p { a href="/blog/" { "All posts" } }

                h3 { "Education" }
                ul .split-list {
                    li {
                        span { "B.Sc. in Mathematics, University of Copenhagen" }
                        span { i { "(2025 – Present)" } }
                    }
                    li {
                        span { "Niels Brock Innovationsgymnasiet" }
                        span { i { "(2021 – 2024)" } }
                    }
                }
            }
        }
        (footer())
    }
}
