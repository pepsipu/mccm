use std::collections::HashMap;
use std::io::Read;

use actix_web::{Result, error::ErrorInternalServerError};
use bollard::{Docker, query_parameters::ListContainersOptionsBuilder};
use bollard::{
    errors::Error as BollardError, query_parameters::DownloadFromContainerOptionsBuilder,
};
use futures_util::StreamExt;
use tar::Archive;

pub struct ServerManager {
    docker: Docker,
}

pub struct ContainerInfo {
    id: String,
    state: String,
}

impl ServerManager {
    pub fn new() -> Self {
        Self {
            docker: Docker::connect_with_defaults().expect("failed to create docker connection"),
        }
    }

    pub async fn download_server_icon(&self, container_id: &str) -> Result<Option<Vec<u8>>> {
        const SERVER_ICON_PATH: &str = "/data/server-icon.png";

        let options = DownloadFromContainerOptionsBuilder::new()
            .path(SERVER_ICON_PATH)
            .build();
        let mut stream = self
            .docker
            .download_from_container(container_id, Some(options));

        let mut tar_bytes = Vec::new();
        while let Some(next) = stream.next().await {
            match next {
                Ok(chunk) => tar_bytes.extend_from_slice(&chunk),
                Err(BollardError::DockerResponseServerError {
                    status_code: 404, ..
                }) => {
                    return Ok(None);
                }
                Err(err) => return Err(ErrorInternalServerError(err)),
            }
        }

        read_tar_single_file(&tar_bytes)
    }

    pub async fn states_by_project(&self) -> Result<HashMap<String, ContainerInfo>> {
        let mut filters = HashMap::new();
        filters.insert("label", vec!["com.docker.compose.service=mc".to_string()]);

        let containers = self
            .docker
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
}

impl ContainerInfo {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn state(&self) -> &str {
        &self.state
    }
}

fn read_tar_single_file(tar_bytes: &[u8]) -> Result<Option<Vec<u8>>> {
    let mut archive = Archive::new(std::io::Cursor::new(tar_bytes));
    let mut entries = archive.entries()?;

    if let Some(entry) = entries.next() {
        let mut out = Vec::new();
        entry?.read_to_end(&mut out)?;
        return Ok(Some(out));
    }

    Ok(None)
}
