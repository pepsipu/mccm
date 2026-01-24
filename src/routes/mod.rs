use actix_web::web;

pub mod create;
pub mod home;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.configure(create::configure).service(home::home);
}
