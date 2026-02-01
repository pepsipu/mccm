use maud::{Markup, html};

use super::motd;

pub fn server_card(name: &str, state: &str, motd_text: &str, icon_url: &str) -> Markup {
    html! {
        .card {
            img.server-icon
                src=(icon_url)
                width="64"
                height="64"
                alt="" {}
            div {
                .server-card-header {
                    strong { (name) }
                    span { (state) }
                }
                .mc-motd { (motd::render_motd(motd_text)) }
            }
        }
    }
}
