use actix_web::{
    HttpResponse, Result, get,
    http::header,
    web::{Bytes, Path},
};

use crate::server;

const DEFAULT_SERVER_ICON: &[u8] = include_bytes!("../../../static/unknown_server.png");

#[get("/{server_name}/icon")]
async fn server_icon(server_name: Path<String>) -> Result<HttpResponse> {
    let server_name = server_name.into_inner();
    let icon_url = server::read_mc_env(&server_name)
        .ok()
        .and_then(|env| env.get("ICON").cloned())
        .unwrap_or_default();
    if icon_url.starts_with("http://") || icon_url.starts_with("https://") {
        return Ok(HttpResponse::Found()
            .insert_header((header::LOCATION, icon_url))
            .finish());
    }

    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .body(Bytes::from_static(DEFAULT_SERVER_ICON)))
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(server_icon);
}
