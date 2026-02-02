use std::collections::HashMap;

use bollard::Docker;
use bollard::models::ContainerSummary;
use bollard::query_parameters::ListContainersOptionsBuilder;

use crate::server::{MINECRAFT_SERVER_IMAGE, PROJECT_LABEL_KEY};

pub struct ServerSummary {
    pub project: String,
    pub id: String,
    pub state: String,
}

pub async fn list_servers(docker: &Docker) -> anyhow::Result<Vec<ServerSummary>> {
    let containers = list_container_summaries(docker).await?;
    Ok(containers.into_iter().filter_map(extract_summary).collect())
}

async fn list_container_summaries(
    docker: &Docker,
) -> anyhow::Result<Vec<ContainerSummary>> {
    let filters = HashMap::from([("ancestor", vec![MINECRAFT_SERVER_IMAGE.to_string()])]);

    Ok(docker
        .list_containers(Some(
            ListContainersOptionsBuilder::new()
                .all(true)
                .filters(&filters)
                .build(),
        ))
        .await?)
}

fn extract_summary(mut container: ContainerSummary) -> Option<ServerSummary> {
    let mut labels = container.labels.take()?;
    let project = labels.remove(PROJECT_LABEL_KEY)?;
    let id = container.id.take()?;
    let state = container.state.take()?.to_string();

    Some(ServerSummary { project, id, state })
}
