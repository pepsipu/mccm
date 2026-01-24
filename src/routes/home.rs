use actix_web::{Result, error::ErrorInternalServerError, get};
use maud::{Markup, html};

use crate::{components, compose};

#[get("/")]
pub async fn home() -> Result<Markup> {
    let servers = compose::list_servers().map_err(ErrorInternalServerError)?;

    Ok(components::page(html! {
        @for name in servers {
            (components::card(name.as_str(), "Placeholder description."))
        }
        (components::create_server_card())
    }))
}
