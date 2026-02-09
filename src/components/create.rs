use maud::{Markup, html};

use crate::components::env_editor;
use crate::modrinth::Project;

pub fn create_page(
    q: Option<&str>,
    error: Option<&str>,
    modrinth_error: bool,
    modpacks: &[Project],
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
            (modpack_card(
                Some(&format!("/create/modrinth/{}", pack.slug)),
                false,
                &pack.title,
                &pack.description,
                pack.icon_url.as_deref(),
                pack.downloads,
                &pack.updated,
            ))
        }
    }
}

pub fn modpack_create_page(project: &Project) -> Markup {
    let env = default_modpack_env(project);

    html! {
        h2 { "Create from Modrinth modpack" }
        (modpack_card(
            Some(&format!("https://modrinth.com/modpack/{}", project.slug)),
            true,
            &project.title,
            &project.description,
            project.icon_url.as_deref(),
            project.downloads,
            &project.updated,
        ))
        (create_form(&project.slug, &env))
    }
}

fn modpack_card(
    href: Option<&str>,
    new_tab: bool,
    title: &str,
    description: &str,
    icon_url: Option<&str>,
    downloads: u64,
    date: &str,
) -> Markup {
    let downloads_badge = format!("{} downloads", downloads);
    let date_badge = date_part(date);

    let body = html! {
        @if let Some(icon_url) = icon_url {
            img src=(icon_url) width="64px" height="64px" {}
        }

        div .card-body {
            div .card-title {
                strong { (title) }
                span .badges {
                    span .badge { (downloads_badge) }
                    span .badge { (date_badge) }
                }
            }
            div { (description) }
        }
    };

    html! {
        @if let Some(href) = href {
            @if new_tab {
                a .card href=(href) target="_blank" rel="noopener noreferrer" { (body) }
            } @else {
                a .card href=(href) { (body) }
            }
        } @else {
            div .card { (body) }
        }
    }
}

fn create_form(default_name: &str, env: &[(String, String)]) -> Markup {
    html! {
        div {
            h3 { "Create server" }
            (env_editor::env_form(
                "/create",
                env,
                html! {
                    .env-row {
                        input
                            type="text"
                            name="name"
                            placeholder="Server name"
                            value=(default_name) {}
                    }
                },
                "Create",
            ))
        }
    }
}

fn default_modpack_env(project: &Project) -> Vec<(String, String)> {
    let mut env = vec![
        ("EULA".to_string(), "TRUE".to_string()),
        ("TYPE".to_string(), "MODRINTH".to_string()),
        ("MODRINTH_MODPACK".to_string(), project.slug.to_string()),
        ("MOTD".to_string(), project.title.to_string()),
    ];

    if let Some(icon_url) = &project.icon_url {
        env.push(("ICON".to_string(), icon_url.to_string()));
    }

    env
}

fn date_part(s: &str) -> &str {
    s.split_once('T').map(|(d, _)| d).unwrap_or(s)
}
