use maud::{Markup, html};

use super::motd;

pub fn server_card(name: &str, state: &str, motd_text: &str, icon_url: &str) -> Markup {
    html! {
        .card.server-card {
            .server-card-header {
                strong { (name) }
                span { (state) }
            }
            .server-card-content {
                img.server-icon
                    src=(icon_url)
                    width="64"
                    height="64"
                    alt="" {}
                div class="mc-motd" { (motd::render_motd(motd_text)) }
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
