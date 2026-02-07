use actix_web::{Result, get, web::Path};
use maud::Markup;

use crate::{components, server};

const DEFAULT_MOTD: &str = "A Minecraft server";

#[get("/{server_name}/motd")]
async fn server_motd(server_name: Path<String>) -> Result<Markup> {
    let server_name = server_name.into_inner();
    let motd = server::read_mc_env(&server_name)
        .ok()
        .and_then(|env| env.get("MOTD").cloned())
        .unwrap_or_else(|| DEFAULT_MOTD.to_string());

    Ok(components::motd::render_motd(&motd))
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(server_motd);
}
