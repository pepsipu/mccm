use actix_files::Files;
use actix_web::{App, HttpServer, web::Data};

use crate::manager::ServerManager;

mod components;
mod compose;
mod manager;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let manager = ServerManager::new();
        App::new()
            .app_data(Data::new(manager))
            .service(Files::new("/static", "./static"))
            .configure(routes::configure)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
