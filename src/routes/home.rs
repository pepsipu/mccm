use std::collections::HashMap;

use actix_web::{Result, error::ErrorInternalServerError, get, web::ThinData};
use bollard::{Docker, query_parameters::ListContainersOptionsBuilder};
use maud::{Markup, html};

use crate::{components, compose};

async fn states_by_project(docker: &Docker) -> Result<HashMap<String, String>> {
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

        let state = container
            .state
            .ok_or_else(|| ErrorInternalServerError("container missing state"))?;
        out.insert(project, state.to_string());
    }

    Ok(out)
}

#[get("/")]
pub async fn home(ThinData(docker): ThinData<Docker>) -> Result<Markup> {
    let servers = compose::list_servers().map_err(ErrorInternalServerError)?;
    let states = states_by_project(&docker).await?;

    Ok(components::page(html! {
        @for name in servers {
            @let state = match states.get(&name) {
                Some(state) => state.as_str(),
                None => "not created",
            };
            (components::card(name.as_str(), state))
        }
        (components::create_server_card())
    }))
}
