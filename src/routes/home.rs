use actix_web::{Result, error::ErrorInternalServerError, get, web::Data};
use bollard::Docker;
use maud::{Markup, html};

use crate::{components, compose, server};

const NOT_CREATED: &str = "not created";

#[get("/")]
pub async fn home(docker: Data<Docker>) -> Result<Markup> {
    let servers = compose::list_servers().map_err(ErrorInternalServerError)?;
    let docker_servers = server::list_servers(&docker)
        .await
        .map_err(ErrorInternalServerError)?;
    let docker_servers = docker_servers
        .into_iter()
        .map(|s| (s.project, s.state))
        .collect::<std::collections::HashMap<_, _>>();

    Ok(components::page(html! {
        @for name in servers {
            @let state = docker_servers.get(&name).map(String::as_str).unwrap_or(NOT_CREATED);
            @let icon_url = format!("/server/{}/icon", name);
            @let motd_url = format!("/server/{}/motd", name);
            (components::server_card(name.as_str(), state, &icon_url, &motd_url))
        }
    }))
}
