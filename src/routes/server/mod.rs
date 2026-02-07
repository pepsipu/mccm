use actix_web::web;

mod icon;
mod motd;
mod page;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/server")
            .configure(icon::configure)
            .configure(motd::configure)
            .configure(page::configure),
    );
}
