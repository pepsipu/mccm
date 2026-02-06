use actix_web::web;
use bollard::Docker;

use crate::server;

mod icon;
mod motd;
mod page;

async fn container_id_for_server(docker: &Docker, server_name: &str) -> Option<String> {
    server::get_server(docker, server_name)
        .await
        .ok()
        .flatten()
        .and_then(|s| s.container_id)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/server")
            .configure(icon::configure)
            .configure(motd::configure)
            .configure(page::configure),
    );
}
