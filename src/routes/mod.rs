use actix_web::web;

pub mod create;
pub mod home;
pub mod icon;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.configure(create::configure)
        .service(home::home)
        .service(icon::server_icon);
}
