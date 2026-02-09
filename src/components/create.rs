use maud::{Markup, html};

use crate::modrinth::ProjectHit;

pub fn create_page(
    q: Option<&str>,
    error: Option<&str>,
    modrinth_error: bool,
    modpacks: &[ProjectHit],
) -> Markup {
    html! {
        h2 { "Create server" }

        @match error {
            Some("exists") => p { "That server name already exists." },
            Some("invalid") => p { "Invalid server name." },
            _ => {}
        }

        form method="post" action="/create" {
            label for="name" { "Name" }
            input type="text" name="name" id="name" {}
            button type="submit" { "Create blank" }
        }

        h2 { "Create from Modrinth modpack" }
        form method="get" action="/create" {
            label for="q" { "Search" }
            input type="text" name="q" id="q" value=(q.unwrap_or("")) {}
            button type="submit" { "Search" }
        }

        @if modrinth_error {
            p { "Failed to load modpacks." }
        } @else if modpacks.is_empty() {
            p { "No modpacks found." }
        }

        @for pack in modpacks {
            (modpack_card(pack))
        }
    }
}

fn modpack_card(pack: &ProjectHit) -> Markup {
    let id = format!("name-{}", pack.project_id);
    let downloads_badge = format!("{} downloads", pack.downloads);
    let date_badge = format!("updated {}", date_part(&pack.date_modified));

    html! {
        div .card {
            @if let Some(icon_url) = &pack.icon_url {
                img src=(icon_url) width="64px" height="64px" {}
            }

            div .card-body {
                div {
                    div .card-title {
                        strong { (pack.title) }
                        span .badges {
                            span .badge { (downloads_badge) }
                            span .badge { (date_badge) }
                        }
                    }
                    div { (pack.description) }
                }

                form method="post" action="/create" {
                    (env_pair("TYPE", "MODRINTH"))
                    (env_pair("MODRINTH_MODPACK", &pack.slug))
                    (env_pair("MOTD", &pack.title))

                    @if let Some(icon_url) = &pack.icon_url {
                        (env_pair("ICON", icon_url))
                    }

                    label for=(id) { "Server name" }
                    input type="text" name="name" id=(id) value=(pack.slug) {}

                    button type="submit" { "Create" }
                }
            }
        }
    }
}

fn env_pair(key: &str, value: &str) -> Markup {
    html! {
        input type="hidden" name="key" value=(key) {}
        input type="hidden" name="value" value=(value) {}
    }
}

fn date_part(s: &str) -> &str {
    s.split_once('T').map(|(d, _)| d).unwrap_or(s)
}
