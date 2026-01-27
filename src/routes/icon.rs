use actix_web::{
    HttpResponse, Result, get,
    web::{Bytes, Data, Path},
};

const DEFAULT_SERVER_ICON: &[u8] = include_bytes!("../../static/unknown_server.png");

use crate::manager::ServerManager;

#[get("/icon/{server_name}")]
pub async fn server_icon(
    manager: Data<ServerManager>,
    server_name: Path<String>,
) -> Result<HttpResponse> {
    let server_name = server_name.into_inner();
    let record = manager.record(&server_name).await.ok_or_else(|| {
        actix_web::error::ErrorNotFound(format!("server '{server_name}' not found"))
    })?;

    let icon_bytes = match record.icon_png() {
        Some(icon_png) => Bytes::copy_from_slice(icon_png.as_slice()),
        None => Bytes::from_static(DEFAULT_SERVER_ICON),
    };

    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .body(icon_bytes))
}
