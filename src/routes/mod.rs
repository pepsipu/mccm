use actix_web::web;

pub mod create;
pub mod home;
pub mod server;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.configure(create::configure)
        .service(home::home)
        .configure(server::configure);
}
