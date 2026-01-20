use actix_web::{App, HttpServer, Result, get};
Z

use maud::{Markup, html};

mod components;
mod containers;

#[get("/")]
async fn servers() -> Result<Markup> {
    let containers = containers::get().await;
    Ok(components::page(html! {
        @for name in containers {
            (components::card("[icon]", name.as_str(), "Placeholder description."))
        }
    }))
}

#[actix_web::main] 
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(servers))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
