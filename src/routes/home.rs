use std::collections::HashMap;

use actix_web::{Result, error::ErrorInternalServerError, get, web::ThinData};
use bollard::{Docker, query_parameters::ListContainersOptionsBuilder};
use maud::{Markup, html};

use crate::{components, compose};

struct ContainerInfo {
    id: String,
    state: String,
}

async fn states_by_project(docker: &Docker) -> Result<HashMap<String, ContainerInfo>> {
    let mut filters = HashMap::new();
    filters.insert("label", vec!["com.docker.compose.service=mc".to_string()]);

    let containers = docker
        .list_containers(Some(
            ListContainersOptionsBuilder::new()
                .all(true)
                .filters(&filters)
                .build(),
        ))
        .await
        .map_err(ErrorInternalServerError)?;

    let mut out = HashMap::new();
    for container in containers {
        let project = container
            .labels
            .as_ref()
            .and_then(|m| m.get("com.docker.compose.project"))
            .cloned();
        let Some(project) = project else { continue };

        let Some(id) = container.id else { continue };
        let state = container
            .state
            .ok_or_else(|| ErrorInternalServerError("container missing state"))?;
        out.insert(
            project,
            ContainerInfo {
                id,
                state: state.to_string(),
            },
        );
    }

    Ok(out)
}

#[get("/")]
pub async fn home(ThinData(docker): ThinData<Docker>) -> Result<Markup> {
    let servers = compose::list_servers().map_err(ErrorInternalServerError)?;
    let states = states_by_project(&docker).await?;

    Ok(components::page(html! {
        @for name in servers {
            @let info = states.get(&name);
            @let state = info.map(|info| info.state.as_str()).unwrap_or("not created");
            @let icon_url = info.map(|info| format!("/icon/{}", info.id));
            (components::card(name.as_str(), state, icon_url.as_deref()))
        }
        (components::create_server_card())
    }))
}
