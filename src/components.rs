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

pub fn server_card(name: &str, state: &str, motd: &str, icon_url: &str) -> Markup {
    html! {
        .card {
            img.server-icon
                src=(icon_url)
                width="64"
                height="64"
                alt="" {}
            .card-body {
                div { (name) }
                div { (motd) }
                div { (state) }
            }
        }
    }
}

pub fn create_server_card() -> Markup {
    html! {
        a href="/create" {
            div class="card create-server-card" {
                .card-body {
                    div { "Create server" }
                    div { "todo" }
                }
            }
        }
    }
}
