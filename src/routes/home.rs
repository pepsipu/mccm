use actix_web::{Result, error::ErrorInternalServerError, get, web::ThinData};
use bollard::Docker;
use maud::{Markup, html};

use crate::{components, server};

#[get("/")]
pub async fn home(ThinData(docker): ThinData<Docker>) -> Result<Markup> {
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
