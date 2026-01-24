use maud::{DOCTYPE, Markup, html};

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

pub fn card(name: &str, description: &str) -> Markup {
    html! {
        .card {
            div { (name) }
            div { (description) }
        }
    }
}

pub fn create_server_card() -> Markup {
    html! {
        a href="/create" {
            (card("Create server", "todo"))
        }
    }
}
