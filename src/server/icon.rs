use bollard::Docker;

const SERVER_ICON_PATH: &str = "/data/server-icon.png";

pub async fn download_server_icon(
    docker: &Docker,
    container_id: &str,
) -> anyhow::Result<Option<Vec<u8>>> {
    super::file::download_file_from_container(docker, container_id, SERVER_ICON_PATH).await
}
