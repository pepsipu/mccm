use actix_web::{
    HttpResponse, Result, get,
    web::{Data, Path},
};

const DEFAULT_SERVER_ICON: &[u8] = include_bytes!("../../static/unknown_server.png");

use crate::manager::ServerManager;

#[get("/icon/{container_id}")]
pub async fn server_icon(
    manager: Data<ServerManager>,
    container_id: Path<String>,
) -> Result<HttpResponse> {
    let container_id = container_id.into_inner();
    let icon_bytes = match manager.download_server_icon(&container_id).await? {
        Some(bytes) => bytes,
        None => DEFAULT_SERVER_ICON.to_vec(),
    };

    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .body(icon_bytes))
}
