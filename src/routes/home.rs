use actix_web::{Result, error::ErrorInternalServerError, get, web::Data};
use bollard::Docker;
use maud::{Markup, html};

use crate::{components, compose, server};

const DEFAULT_MOTD: &str = "A Minecraft server";

#[get("/")]
pub async fn home(docker: Data<Docker>) -> Result<Markup> {
    let servers = compose::list_servers().map_err(ErrorInternalServerError)?;
    let docker_servers = server::list_servers(&docker)
        .await
        .map_err(ErrorInternalServerError)?;
    let docker_servers = docker_servers
        .into_iter()
        .map(|s| (s.project, (s.id, s.state)))
        .collect::<std::collections::HashMap<_, _>>();

    let mut view = Vec::new();
    for name in servers {
        let (state, motd) = match docker_servers.get(&name) {
            Some((container_id, state)) => {
                let props = server::download_server_properties(&docker, container_id)
                    .await
                    .unwrap_or_default();
                let motd = props
                    .get("motd")
                    .map(String::as_str)
                    .unwrap_or(DEFAULT_MOTD)
                    .to_string();
                (state.as_str().to_string(), motd)
            }
            None => ("not created".to_string(), DEFAULT_MOTD.to_string()),
        };

        view.push((name, state, motd));
    }

    Ok(components::page(html! {
        @for (name, state, motd) in view {
            @let icon_url = format!("/icon/{}", name);
            (components::server_card(name.as_str(), state.as_str(), motd.as_str(), &icon_url))
        }
    }))
}
