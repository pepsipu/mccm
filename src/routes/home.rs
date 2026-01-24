use std::collections::HashMap;

use actix_web::{Result, error::ErrorInternalServerError, get, web::ThinData};
use bollard::{Docker, query_parameters::ListContainersOptionsBuilder};
use maud::{Markup, html};

use crate::{components, compose};

fn state_str(state: bollard::models::ContainerSummaryStateEnum) -> &'static str {
    match state {
        bollard::models::ContainerSummaryStateEnum::CREATED => "created",
        bollard::models::ContainerSummaryStateEnum::RUNNING => "running",
        bollard::models::ContainerSummaryStateEnum::PAUSED => "paused",
        bollard::models::ContainerSummaryStateEnum::RESTARTING => "restarting",
        bollard::models::ContainerSummaryStateEnum::EXITED => "exited",
        bollard::models::ContainerSummaryStateEnum::REMOVING => "removing",
        bollard::models::ContainerSummaryStateEnum::DEAD => "dead",
        bollard::models::ContainerSummaryStateEnum::EMPTY => "",
    }
}

async fn project_state(docker: &Docker, project: &str) -> Result<String> {
    let mut filters = HashMap::new();
    filters.insert(
        "label",
        vec![
            format!("com.docker.compose.project={project}"),
            "com.docker.compose.service=mc".to_string(),
        ],
    );

    let mut containers = docker
        .list_containers(Some(
            ListContainersOptionsBuilder::new()
                .all(true)
                .filters(&filters)
                .build(),
        ))
        .await
        .map_err(ErrorInternalServerError)?;

    let Some(container) = containers.pop() else {
        return Ok("not created".to_string());
    };

    let state = container
        .state
        .ok_or_else(|| ErrorInternalServerError("container missing state"))?;
    Ok(state_str(state).to_string())
}

#[get("/")]
pub async fn home(ThinData(docker): ThinData<Docker>) -> Result<Markup> {
    let servers = compose::list_servers().map_err(ErrorInternalServerError)?;
    let mut server_states = Vec::with_capacity(servers.len());
    for name in servers {
        let state = project_state(&docker, name.as_str()).await?;
        server_states.push((name, state));
    }

    Ok(components::page(html! {
        @for (name, state) in server_states {
            (components::card(name.as_str(), state.as_str()))
        }
        (components::create_server_card())
    }))
}
