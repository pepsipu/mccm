use actix_files::Files;
use actix_web::{App, HttpServer, Result, error::ErrorInternalServerError, get, web::ThinData};
use bollard::Docker;
use maud::{Markup, html};

mod components;
mod compose;
mod server;

#[get("/")]
async fn home(ThinData(docker): ThinData<Docker>) -> Result<Markup> {
    let containers = server::list(docker)
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(components::page(html! {
        @for name in containers {
            (components::card(name.as_str(), "Placeholder description."))
        }
        (components::create_server_card())
    }))
}

#[get("/create")]
async fn create() -> Result<Markup> {
    Ok(components::page(html! {
        form method="post" action="/create" {
            div {
                label for="name" { "Name" }
                input type="text" name="name" id="name" {}
            }
            div {
                label for="type" { "Type" }
                input type="text" name="type" id="type" {}
            }
            button type="submit" { "Create" }
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
            .service(home)
            .service(create)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
