use actix_files::Files;
use actix_web::{App, HttpServer, Result, get, web::ThinData, error::ErrorInternalServerError};
use bollard::Docker;
use maud::{Markup, html};

mod components;
mod containers;

#[get("/")]
async fn servers(ThinData(docker): ThinData<Docker>) -> Result<Markup> {
    let containers = containers::list(docker).await.map_err(ErrorInternalServerError)?;
    Ok(components::page(html! {
        @for name in containers {
            (components::card("[icon]", name.as_str(), "Placeholder description."))
        }
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let docker = Docker::connect_with_local_defaults().expect("failed to create docker client");

    HttpServer::new(move || {
        App::new()
            .app_data(ThinData(docker.clone()))
            .service(Files::new("/static", "./static"))
            .service(servers)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
