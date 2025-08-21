use maud::{html, Markup, DOCTYPE};
use chrono::Datelike;

pub fn header(title: &str, description: &str) -> Markup {
    html! {
        (DOCTYPE)

        meta charset="UTF-8";
        meta content="width=device-width,initial-scale=1" name="viewport";

        title { (title) };
        meta content=(title) property="og:title";

        meta content=(description) name="description";
        meta content=(description) property="og:description";

        link inline rel="stylesheet" href="/assets/style.css";

        link rel="canonical" href="https://bergsoe.net/";

        // link rel="icon" href="data:,";
        link rel="icon" href="/assets/favicon.svg" type="image/svg+xml";

        header {
            a href="/#h" { "Vilhelm Bergsøe" }
            nav {
                a href="/#b" { "Blog" }
                a href="/#g" { "Contact" }
            }
        }
    }
}

pub fn footer() -> Markup {
    html! {
        footer {
            "© " (chrono::Utc::now().year().to_string()) " " a href="https://github.com/vilhelmbergsoe" { "Vilhelm Bergsøe" }
        }
    }
}
