use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use maud::{html, Markup, PreEscaped, DOCTYPE};

use crate::AppState;

fn header() -> Markup {
    html! {
        (DOCTYPE)
        title { "Vilhelm Bergsøe" };
        meta content="width=device-width,initial-scale=1" name="viewport";
        meta content="Vilhelm Bergsøe" property="og:title";
        meta content="Vilhelm Bergsøe" property="description";
        meta content="Vilhelm Bergsøe" property="og:description";
        link rel="stylesheet" href="/assets/style.css";
        header {
            a href="/#h" { "Vilhelm Bergsøe" }
            nav {
                a href="/#b" { "Blog" }
                a href="/#g" { "Contact" }
            }
        }
    }
}

pub async fn root(State(state): State<AppState>) -> Markup {
    html! {
        (header())
        main {
            section #b {
                h2 { "Blog" }
                ul {
                    @for blogpost in &state.blogposts {
                        @if blogpost.archived == false {
                            li {
                                (blogpost.date.format("%d-%m-%Y")) " - "
                                a href=(format!("/blog/{}", blogpost.url)) { (blogpost.title) }
                            }
                        }
                    }
                }
            }
            section #g {
                h2 { "Contact" }
                ul {
                    li { "email me at " a href="mailto:vilhelm@bergsoe.net" { "vilhelm@bergsoe.net"} }
                    li { "my" a href="/assets/gpg.txt" { " GPG key" } }
                }
            }
            section #h {
                h2 { "Info" }
                p { "Hi there, I am an aspiring software developer, from
                Copenhagen, Denmark, who has a passion for computers and enjoys
                learning about new things and technologies" }
                br;
                ul {
                    li {
                        a href="/assets/cv.pdf" { "CV" }
                    }
                    li {
                        a href="https://github.com/vilhelmbergsoe" { "GitHub" }
                    }
                }

                h3 { "Projects" }
                br;
                ul {
                    li { a href="https://github.com/vilhelmbergsoe/asciicam" { "asciicam" } " - An ascii webcam in your console" }
                    li { a href="https://github.com/vilhelmbergsoe/snake" { "snake" } " - A CLI snake clone" }
                    li { a href="https://github.com/vilhelmbergsoe/mazegen" { "mazegen" } " - A simple maze generator that uses recursive backtracking" }
                    li { a href="https://github.com/vilhelmbergsoe/site" { "site" } " - My personal website with blog functionality" }
                }

                h3 { "Skills" }
                br;
                ul {
                    li { "Programming in Go" img .go-icon src="/assets/go.svg"; ", Rust 🦀, C, Zig and JavaScript" }
                    li { "Linux system administration" }
                    li { "Web development technologies: Docker, Git HTML, CSS, SQL and React" }
                    li { "Some experience contributing to open source projects" }
                    li { "Search engine ninja 🥷" }
                    li { "Fast learner" }
                }

                h3 { "Education" }
                br;
                ul {
                    li { "Niels Brock Innovationsgymnasiet @ 2021-2024" }
                }
            }
        }
    }
}

pub async fn handle_404() -> (StatusCode, PreEscaped<String>) {
    (
        StatusCode::NOT_FOUND,
        html! {
            (header())
            p { "404 Not found" }
        },
    )
}

pub async fn handle_blog(
    Path(url): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let blogpost = state.blogposts.iter().find(|blogpost| blogpost.url == url);

    match blogpost {
        Some(blogpost) => (
            StatusCode::OK,
            html! {
                (header())
                main {
                    section #h {
                        div .blogpost {
                            h2 { (blogpost.title) }
                            span style="opacity: 0.7;" { (blogpost.date.format("%d-%m-%Y")) }
                            br;
                            p {
                                (PreEscaped(blogpost.content.to_string()))
                            }
                        }
                    }
                }
            },
        ),
        None => handle_404().await,
    }
}
