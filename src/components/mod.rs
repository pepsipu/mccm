use maud::{DOCTYPE, Markup, html};

pub mod card;
pub mod motd;

pub use card::{create_server_card, server_card};

pub fn page(body: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                script src="/static/htmx.min.js" {}
                link rel="stylesheet" href="/static/style.css" {}
            }
            body {
                h1 { "mccm" }
                (body)
            }
        }
    }
}
