use actix_web::{
    HttpResponse, Result, get,
    web::{Bytes, Data, Path},
};
use bollard::Docker;

use crate::server;

const DEFAULT_SERVER_ICON: &[u8] = include_bytes!("../../../static/unknown_server.png");

#[get("/{server_name}/icon")]
async fn server_icon(docker: Data<Docker>, server_name: Path<String>) -> Result<HttpResponse> {
    let server_name = server_name.into_inner();
    let icon_bytes = match super::container_id_for_project(&docker, &server_name).await {
        Some(container_id) => match server::download_server_icon(&docker, &container_id).await {
            Ok(Some(png)) => Bytes::from(png),
            _ => Bytes::from_static(DEFAULT_SERVER_ICON),
        },
        None => Bytes::from_static(DEFAULT_SERVER_ICON),
    };

    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .body(icon_bytes))
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(server_icon);
}
