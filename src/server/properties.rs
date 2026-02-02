use bollard::Docker;
use encoding_rs::UTF_8;
use java_properties::PropertiesIter;
use std::collections::HashMap;

use crate::server::ServerStateError;

const SERVER_PROPERTIES_PATH: &str = "/data/server.properties";

pub async fn download_server_properties(
    docker: &Docker,
    container_id: &str,
) -> Result<HashMap<String, String>, ServerStateError> {
    let Some(file_bytes) =
        super::file::download_file_from_container(docker, container_id, SERVER_PROPERTIES_PATH)
            .await?
    else {
        return Ok(HashMap::new());
    };

    let mut props = HashMap::new();
    let mut iter = PropertiesIter::new_with_encoding(std::io::Cursor::new(file_bytes), UTF_8);
    iter.read_into(|k, v| {
        props.insert(k, v);
    })
    .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))?;

    Ok(props)
}
