use actix_web::{
    HttpResponse, Result, get,
    web::{Bytes, Data, Path},
};
use bollard::Docker;

const DEFAULT_SERVER_ICON: &[u8] = include_bytes!("../../static/unknown_server.png");

use crate::server;

#[get("/icon/{server_name}")]
pub async fn server_icon(docker: Data<Docker>, server_name: Path<String>) -> Result<HttpResponse> {
    let server_name = server_name.into_inner();
    let icon_bytes = match server::list_servers(&docker).await {
        Ok(servers) => match servers
            .into_iter()
            .find(|s| s.project == server_name)
            .map(|s| s.id)
        {
            Some(container_id) => {
                match server::download_server_icon(&docker, &container_id).await {
                    Ok(Some(png)) => Bytes::from(png),
                    _ => Bytes::from_static(DEFAULT_SERVER_ICON),
                }
            }
            None => Bytes::from_static(DEFAULT_SERVER_ICON),
        },
        Err(_) => Bytes::from_static(DEFAULT_SERVER_ICON),
    };

    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .body(icon_bytes))
}
