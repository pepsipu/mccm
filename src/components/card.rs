use maud::{Markup, html};

pub fn server_card(name: &str, state: &str) -> Markup {
    let server_page = format!("/server/{}", name);
    let icon_url = format!("{}/icon", server_page);
    let motd_url = format!("{}/motd", server_page);

    html! {
        a .card href=(server_page) {
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
