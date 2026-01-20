use bollard::{Docker, query_parameters::ListContainersOptionsBuilder};
use std::collections::HashMap;

pub async fn list() -> Vec<String> {
    let docker = match Docker::connect_with_local_defaults() {
        Ok(docker) => docker,
        Err(_) => return Vec::new(),
    };

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

    match docker.list_containers(options).await {
        Ok(containers) => containers
            .into_iter()
            .filter_map(|container| {
                container
                    .names
                    .and_then(|names| names.into_iter().next())
            })
            .collect(),
        Err(_) => Vec::new(),
    }
}
