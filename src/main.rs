use actix_files::Files;
use actix_web::{App, HttpServer, web::Data};
use bollard::Docker;

mod components;
mod compose;
mod routes;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let docker =
        Data::new(Docker::connect_with_defaults().expect("failed to create docker connection"));
    HttpServer::new(move || {
        App::new()
            .app_data(docker.clone())
            .service(Files::new("/static", "./static"))
            .configure(routes::configure)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
