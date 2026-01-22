use bollard::{Docker, errors::Error, query_parameters::ListContainersOptionsBuilder};
use std::collections::HashMap;

pub async fn list(docker: Docker) -> Result<Vec<String>, Error> {
    let mut filters = HashMap::new();
    filters.insert(
        "ancestor".to_string(),
        vec!["itzg/minecraft-server".to_string()],
    );

    let options = Some(
        ListContainersOptionsBuilder::new()
            .all(true)
            .filters(&filters)
            .build(),
    );

    let containers = docker.list_containers(options).await?;

    let names = containers
        .into_iter()
        .filter_map(|container| container.names.and_then(|names| names.into_iter().next()))
        .collect();

    Ok(names)
}
