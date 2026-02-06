use maud::{DOCTYPE, Markup, html};

pub mod card;
pub mod env_editor;
pub mod motd;

pub use card::server_card;
pub use env_editor::env_editor;

pub fn header() -> Markup {
    html! {
        .header {
            h1 {
                a href="/" { "mccm" }
            }
            nav {
                a .link href="/create" { "create" }
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
