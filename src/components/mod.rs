use maud::{DOCTYPE, Markup, html};

pub mod card;
pub mod motd;

pub use card::server_card;

pub fn header() -> Markup {
    html! {
        div class="header" {
            h1 { "mccm" }
            nav {
                a href="/create" { "create" }
            }
        }
    }
}

pub fn page(body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                script src="/static/htmx.min.js" {}
                link rel="stylesheet" href="/static/style.css" {}
            }
            body {
                (header())
                (body)
            }
        }
    }
}
