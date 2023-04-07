use maud::{html, Markup, DOCTYPE};

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
            "© 2023 " a href="https://github.com/vilhelmbergsoe" { "Vilhelm Bergsøe" } ", Powered by " a href="https://nixos.org" { "Nix" } " ❄️"
        }
    }
}
