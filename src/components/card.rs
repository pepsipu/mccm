use maud::{Markup, html};

use super::motd;

pub fn server_card(name: &str, state: &str, motd_text: &str, icon_url: &str) -> Markup {
    html! {
        .card {
            img
                src=(icon_url)
                alt="" {}
            .card-body {
                span {
                    strong { (name) }
                    span { " " (state) }
                }
                .mc-motd { (motd::render_motd(motd_text)) }
            }
        }
    }
}
