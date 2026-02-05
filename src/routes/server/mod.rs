use actix_web::web;
use bollard::Docker;

use crate::server;

mod icon;
mod motd;
mod page;

async fn container_id_for_project(docker: &Docker, project: &str) -> Option<String> {
    let servers = server::list_servers(docker).await.ok()?;
    servers
        .into_iter()
        .find(|s| s.project == project)
        .map(|s| s.id)
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/server")
            .configure(icon::configure)
            .configure(motd::configure)
            .configure(page::configure),
    );
}
