use actix_files::Files;
use actix_web::{App, HttpServer, web::ThinData};
use bollard::Docker;

mod components;
mod compose;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let docker = Docker::connect_with_local_defaults().expect("failed to create docker client");

    HttpServer::new(move || {
        App::new()
            .app_data(ThinData(docker.clone()))
            .service(Files::new("/static", "./static"))
            .configure(routes::configure)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
