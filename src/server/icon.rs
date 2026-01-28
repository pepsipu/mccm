use bollard::Docker;

use crate::server::ServerStateError;

const SERVER_ICON_PATH: &str = "/data/server-icon.png";

pub async fn download_server_icon(
    docker: &Docker,
    container_id: &str,
) -> Result<Option<Vec<u8>>, ServerStateError> {
    super::container_file::download_single_file_from_container(docker, container_id, SERVER_ICON_PATH)
        .await
}
