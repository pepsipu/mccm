use actix_web::{
    Result, get,
    web::{Data, Path},
};
use bollard::Docker;
use maud::Markup;

use crate::{components, server};

const DEFAULT_MOTD: &str = "A Minecraft server";

#[get("/{server_name}/motd")]
async fn server_motd(docker: Data<Docker>, server_name: Path<String>) -> Result<Markup> {
    let server_name = server_name.into_inner();
    let motd = match super::container_id_for_server(&docker, &server_name).await {
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
    cfg.service(server_motd);
}
