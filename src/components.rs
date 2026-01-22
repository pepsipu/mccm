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

pub fn card(icon: &str, name: &str, description: &str) -> Markup {
    html! {
        table {
            tr {
                td { (icon) }
                td {
                    div { (name) }
                    div { (description) }
                }
            }
        }
    }
}
