use actix_web::{Result, error::ErrorInternalServerError, get, web::Data};
use maud::{Markup, html};

use crate::{components, compose, manager::ServerManager};

#[get("/")]
pub async fn home(manager: Data<ServerManager>) -> Result<Markup> {
    let servers = compose::list_servers().map_err(ErrorInternalServerError)?;
    let states = manager.states_by_project().await?;

    Ok(components::page(html! {
        @for name in servers {
            @let info = states.get(&name);
            @let state = info.map(|info| info.state()).unwrap_or("not created");
            @let icon_url = info.map(|info| format!("/icon/{}", info.id()));
            (components::card(name.as_str(), state, icon_url.as_deref()))
        }
        (components::create_server_card())
    }))
}
