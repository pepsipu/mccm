use std::collections::HashMap;

use bollard::Docker;
use bollard::models::ContainerSummary;
use bollard::query_parameters::ListContainersOptionsBuilder;
use indexmap::IndexMap;

use super::{MINECRAFT_SERVER_IMAGE, PROJECT_LABEL_KEY};
use crate::compose;

const NOT_CREATED: &str = "not created";

#[derive(Clone)]
pub struct ServerState {
    pub container_id: Option<String>,
    pub state: String,
}

pub async fn get_servers(docker: &Docker) -> anyhow::Result<IndexMap<String, ServerState>> {
    let server_names = compose::list_servers()?;

    let mut out: IndexMap<String, ServerState> = server_names
        .into_iter()
        .map(|name| {
            (
                name,
                ServerState {
                    container_id: None,
                    state: NOT_CREATED.to_string(),
                },
            )
        })
        .collect();

    let filters = HashMap::from([("ancestor", vec![MINECRAFT_SERVER_IMAGE.to_string()])]);
    let containers = docker
        .list_containers(Some(
            ListContainersOptionsBuilder::new()
                .all(true)
                .filters(&filters)
                .build(),
        ))
        .await?;

    for container in containers {
        let Some((server, id, state)) = extract_server(container) else {
            continue;
        };
        let Some(entry) = out.get_mut(&server) else {
            continue;
        };

        entry.container_id = Some(id);
        entry.state = state;
    }

    Ok(out)
}

pub async fn get_server(docker: &Docker, server_name: &str) -> anyhow::Result<Option<ServerState>> {
    Ok(get_servers(docker).await?.get(server_name).cloned())
}

fn extract_server(mut container: ContainerSummary) -> Option<(String, String, String)> {
    let mut labels = container.labels.take()?;
    let server = labels.remove(PROJECT_LABEL_KEY)?;
    let id = container.id.take()?;
    let state = container.state.take()?.to_string();
    Some((server, id, state))
}
