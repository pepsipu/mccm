use std::io::Read;

use bollard::Docker;
use bollard::errors::Error as BollardError;
use bollard::query_parameters::DownloadFromContainerOptionsBuilder;
use futures_util::StreamExt;
use tar::Archive;

pub async fn download_file_from_container(
    docker: &Docker,
    container_id: &str,
    path: &str,
) -> anyhow::Result<Option<Vec<u8>>> {
    let options = DownloadFromContainerOptionsBuilder::new()
        .path(path)
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
            Err(err) => return Err(err.into()),
        }
    }

    Ok(read_tar_single_file(&tar_bytes)?)
}

fn read_tar_single_file(tar_bytes: &[u8]) -> anyhow::Result<Option<Vec<u8>>> {
    let mut archive = Archive::new(std::io::Cursor::new(tar_bytes));
    let mut entries = archive.entries()?;

    let Some(entry) = entries.next() else {
        return Ok(None);
    };

    let mut out = Vec::new();
    entry?.read_to_end(&mut out)?;
    return Ok(Some(out));
}
