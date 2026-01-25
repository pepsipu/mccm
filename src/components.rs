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

pub fn card(name: &str, description: &str, icon_url: Option<&str>) -> Markup {
    html! {
        .card {
            @if let Some(icon_url) = icon_url {
                img.server-icon
                    src=(icon_url)
                    width="64"
                    height="64"
                    alt="" {}
            }
            .card-body {
                div { (name) }
                div { (description) }
            }
        }
    }
}

pub fn create_server_card() -> Markup {
    html! {
        a href="/create" {
            (card("Create server", "todo", None))
        }
    }
}
