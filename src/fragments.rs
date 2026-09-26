use maud::{html, Markup, DOCTYPE};

pub fn header(title: &str, description: &str) -> Markup {
    assert!(!title.is_empty());
    assert!(!description.is_empty());

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
            a href="/" { "Vilhelm Bergsøe" }
            nav {
                a href="/blog/" { "Blog" }
            }
        }
    }
}

pub fn footer() -> Markup {
    html! {
        footer {
            div.signet-block {
                img.signet src="/assets/bergsoe.webp" alt="signet";
                hr;
                "© 2026 " a href="https://github.com/vilhelmbergsoe" { "Vilhelm Bergsøe" }
            }
        }
    }
}
