use actix_web::{
    HttpResponse, Result,
    error::ErrorInternalServerError,
    get,
    web::{Path, ThinData},
};
use bollard::Docker;

const DEFAULT_SERVER_ICON: &[u8] = include_bytes!("../../static/unknown_server.png");
use std::io::Read;

use bollard::{
    errors::Error as BollardError, query_parameters::DownloadFromContainerOptionsBuilder,
};
use futures_util::StreamExt;
use tar::Archive;

const SERVER_ICON_PATH: &str = "/data/server-icon.png";

fn read_tar_single_file(tar_bytes: &[u8]) -> Result<Option<Vec<u8>>> {
    let mut archive = Archive::new(std::io::Cursor::new(tar_bytes));
    let mut entries = archive.entries()?;

    if let Some(entry) = entries.next() {
        let mut out = Vec::new();
        entry?.read_to_end(&mut out)?;
        return Ok(Some(out));
    }

    return Ok(None);
}

pub async fn download_server_icon(docker: &Docker, container_id: &str) -> Result<Option<Vec<u8>>> {
    let options = DownloadFromContainerOptionsBuilder::new()
        .path(SERVER_ICON_PATH)
        .build();
    let mut stream = docker.download_from_container(container_id, Some(options));

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

    return read_tar_single_file(&tar_bytes);
}

#[get("/icon/{container_id}")]
pub async fn server_icon(
    ThinData(docker): ThinData<Docker>,
    container_id: Path<String>,
) -> Result<HttpResponse> {
    let container_id = container_id.into_inner();
    let icon_bytes = match download_server_icon(&docker, &container_id).await? {
        Some(bytes) => bytes,
        None => DEFAULT_SERVER_ICON.to_vec(),
    };

    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .body(icon_bytes))
}
