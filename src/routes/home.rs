use actix_web::{Result, error::ErrorInternalServerError, get, web::Data};
use maud::{Markup, html};

use crate::{components, compose, manager::ServerManager};

#[get("/")]
pub async fn home(manager: Data<ServerManager>) -> Result<Markup> {
    let servers = compose::list_servers().map_err(ErrorInternalServerError)?;
    let states = manager.records().await;

    Ok(components::page(html! {
        @for name in servers {
            @let info = states.get(&name);
            @let state = info.map(|record| record.state()).unwrap_or("not created");
            @let motd = info.and_then(|record| record.motd());
            @let icon_url = info.map(|_| format!("/icon/{}", name));
            (components::server_card(name.as_str(), state, motd, icon_url.as_deref()))
        }
        (components::create_server_card())
    }))
}
