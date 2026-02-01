use actix_files::Files;
use actix_web::{App, HttpServer, web::Data};

use crate::manager::ServerManager;

mod components;
mod compose;
mod manager;
mod routes;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let manager = Data::new(ServerManager::new());
    manager::spawn(manager.clone());
    HttpServer::new(move || {
        App::new()
            .app_data(manager.clone())
            .service(Files::new("/static", "./static"))
            .configure(routes::configure)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
