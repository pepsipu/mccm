use maud::{Markup, html};

pub fn server_card(name: &str, state: &str, icon_url: &str, motd_url: &str) -> Markup {
    html! {
        .card {
            img
                src=(icon_url)
                width="64px"
                height="64px" {}
            .card-body {
                span {
                    strong { (name) }
                    span { " " (state) }
                }
                .mc-motd
                    hx-get=(motd_url)
                    hx-trigger="load"
                    hx-swap="innerHTML"
                {}
            }
        }
    }
}
