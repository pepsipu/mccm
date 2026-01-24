use actix_files::Files;
use actix_web::{App, HttpServer};

mod components;
mod compose;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("servers")?;

    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", "./static"))
            .configure(routes::configure)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
