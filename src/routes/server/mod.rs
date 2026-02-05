use actix_web::{
    HttpResponse, Result, get,
    web::{Bytes, Data, Path},
};
use bollard::Docker;
use maud::Markup;

use crate::{components, server};

const DEFAULT_SERVER_ICON: &[u8] = include_bytes!("../../../static/unknown_server.png");
const DEFAULT_MOTD: &str = "A Minecraft server";

async fn container_id_for_project(docker: &Docker, project: &str) -> Option<String> {
    let servers = server::list_servers(docker).await.ok()?;
    servers
        .into_iter()
        .find(|s| s.project == project)
        .map(|s| s.id)
}

#[get("/{server_name}/icon")]
async fn server_icon(docker: Data<Docker>, server_name: Path<String>) -> Result<HttpResponse> {
    let server_name = server_name.into_inner();
    let icon_bytes = match container_id_for_project(&docker, &server_name).await {
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

#[get("/{server_name}/motd")]
async fn server_motd(docker: Data<Docker>, server_name: Path<String>) -> Result<Markup> {
    let server_name = server_name.into_inner();
    let motd = match container_id_for_project(&docker, &server_name).await {
        Some(container_id) => server::download_server_properties(&docker, &container_id)
            .await
            .unwrap_or_default()
            .get("motd")
            .cloned()
            .unwrap_or_else(|| DEFAULT_MOTD.to_string()),
        None => DEFAULT_MOTD.to_string(),
    };

    Ok(components::motd::render_motd(&motd))
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        actix_web::web::scope("/server")
            .service(server_icon)
            .service(server_motd),
    );
}
