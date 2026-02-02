use bollard::Docker;
use encoding_rs::UTF_8;
use java_properties::PropertiesIter;
use std::collections::HashMap;

const SERVER_PROPERTIES_PATH: &str = "/data/server.properties";

pub async fn download_server_properties(
    docker: &Docker,
    container_id: &str,
) -> anyhow::Result<HashMap<String, String>> {
    let Some(file_bytes) =
        super::file::download_file_from_container(docker, container_id, SERVER_PROPERTIES_PATH)
            .await?
    else {
        return Ok(HashMap::new());
    };

    let mut props = HashMap::new();
    // TODO: minecraft 1.20+ writes property files as UTF-8, but older server.properties may need to be parsed as ISO-8859-1
    let mut iter = PropertiesIter::new_with_encoding(std::io::Cursor::new(file_bytes), UTF_8);
    iter.read_into(|k, v| {
        props.insert(k, v);
    })?;

    Ok(props)
}
