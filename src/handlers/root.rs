use axum::{extract::State, response::IntoResponse};
use maud::html;

use crate::fragments::{footer, header};
use crate::AppState;

pub async fn root(State(state): State<AppState>) -> impl IntoResponse {
    html! {
        (header("Vilhelm Bergsøe - Home", "Vilhelm Bergsøe's personal website and blog"))
        main {
            section #b {
                h2 { "Blog " a href="/rss.xml" title="RSS Feed" { img .rss-icon src="/assets/rss.png" alt="rss"; } }
                ul {
                    @for blogpost in &state.blogposts {
                        @if !blogpost.archived {
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
                    li { "my " a href="/assets/gpg.txt" { "GPG key" } }
                }
            }
            section #h {
                h2 { "Info" }
                p { "Hi there! I'm a software developer, from Copenhagen,
                Denmark, with a passion for technology and programming. I love
                to find simple or elegant solutions to complex problems and I'm
                always eager to learn new things." }
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
                    li { a href="https://github.com/vilhelmbergsoe/asciicam" { "asciicam" } " - An ASCII webcam in your console" }
                    li { a href="https://github.com/vilhelmbergsoe/snake" { "snake" } " - A CLI snake clone" }
                    li { a href="https://github.com/vilhelmbergsoe/mazegen" { "mazegen" } " - A simple maze generator that uses recursive backtracking" }
                    li { a href="https://github.com/vilhelmbergsoe/site" { "site" } " - My personal website with blog functionality" }
                }

                h3 { "Skills" }
                br;
                ul {
                    li { "Programming in Go" img .go-icon src="/assets/go.svg" alt="golang gopher"; ", Rust 🦀, C and JavaScript" }
                    li { "Very experienced with Linux 🐧" }
                    li { "Web development technologies: Docker, Git, HTML, CSS, SQL and React" }
                    li { "Experience contributing to open source projects" }
                    li { "Fast learner 📖" }
                }

                h3 { "Education" }
                br;
                ul {
                    li { "Niels Brock Innovationsgymnasiet 2021-2024" }
                }
            }
        }
        (footer())
    }
}
