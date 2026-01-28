use bollard::Docker;

use crate::server::ServerStateError;

const SERVER_PROPERTIES_PATH: &str = "/data/server.properties";

pub async fn download_server_motd(
    docker: &Docker,
    container_id: &str,
) -> Result<Option<String>, ServerStateError> {
    let Some(file_bytes) = super::container_file::download_single_file_from_container(
        docker,
        container_id,
        SERVER_PROPERTIES_PATH,
    )
    .await?
    else {
        return Ok(None);
    };

    let props = java_properties::read(std::io::Cursor::new(file_bytes))
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;

    Ok(props.get("motd").cloned())
}
