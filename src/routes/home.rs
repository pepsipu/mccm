use actix_web::{Result, error::ErrorInternalServerError, get, web::Data};
use bollard::Docker;
use maud::{Markup, html};

use crate::{components, server};

#[get("/")]
pub async fn home(docker: Data<Docker>) -> Result<Markup> {
    let servers = server::get_servers(&docker)
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(components::page(html! {
        @for (name, server) in servers {
            (components::server_card(name.as_str(), server.state.as_str()))
        }
    }))
}
