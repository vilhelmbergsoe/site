use axum::{extract::State, response::IntoResponse};
use chrono::Datelike;
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
                                (blogpost.date.format("D%d-%m-%Y "))
                                a href=(format!("/blog/{}", blogpost.url)) { (blogpost.title) }
                            }
                        }
                    }
                }
            }
            section #g {
                h2 { "Contact" }
                p { "email me at " a href="mailto:vilhelm@bergsoe.net" {"vilhelm@bergsoe.net"} br;
                "my " a href="/assets/gpg.txt" { "GPG key" }
                }

            }
            section #h {
                h2 { "Info" }
		p { "Software Developer from Copenhagen, Denmark 🇩🇰,
		with an interest in programming, economics and
		mathematics. Check out my " a href="/#b" { "blog" } "
		to read about what I'm working on."  }

                ul {
                    li {
                        a href="/assets/cv.pdf" { "CV" }
                    }
                    li {
                        a href="https://github.com/vilhelmbergsoe" { "GitHub" }
                    }
                }

                h3 { "Projects" }
                ul {
                    li { a href="https://github.com/vilhelmbergsoe/asciicam" { "asciicam" } " - An ASCII webcam in your console" }
                    li { a href="https://github.com/vilhelmbergsoe/snake" { "snake" } " - A CLI snake clone" }
                    li { a href="https://github.com/vilhelmbergsoe/mazegen" { "mazegen" } " - A simple maze generator that uses recursive backtracking" }
                    li { a href="https://github.com/vilhelmbergsoe/site" { "site" } " - My personal website with blog functionality" }
                    li { a href="https://github.com/vilhelmbergsoe/teenyfold" { "teenyfold" } " - (WIP) Protein folding / Structure prediction model" }
                }

                h3 { "Skills" }
                ul {
                    li { b { "Programming Languages: " } br;
			  "Go, Rust, C, JavaScript and more"
		    }
		    li { b { "Tools & Technologies: " } br;
			  "Docker, Git, Linux (" ((chrono::Utc::now().year() - 2015).to_string()) "+ years 🐧), HTML, CSS, SQL, React, Nix"
		    }
		    li { b { "Currently learning: " } br;
			 "ML & Data science, Biochemistry"
		    }
                }

                h3 { "Education" }
                ul {
                    li { "Niels Brock Innovationsgymnasiet 2021-2024" }
                }
            }
        }
        (footer())
    }
}
