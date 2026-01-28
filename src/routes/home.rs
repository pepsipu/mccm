use actix_web::{Result, error::ErrorInternalServerError, get, web::Data};
use maud::{Markup, html};

use crate::{components, compose, manager::ServerManager};

const DEFAULT_MOTD: &str = "A Minecraft server";

#[get("/")]
pub async fn home(manager: Data<ServerManager>) -> Result<Markup> {
    let servers = compose::list_servers().map_err(ErrorInternalServerError)?;
    let states = manager.records().await;

    Ok(components::page(html! {
        @for name in servers {
            @let info = states.get(&name);
            @let state = info.map(|record| record.state()).unwrap_or("not created");
            @let motd = info.and_then(|r| r.properties().get("motd").map(String::as_str)).unwrap_or(DEFAULT_MOTD);
            @let icon_url = format!("/icon/{}", name);
            (components::server_card(name.as_str(), state, motd, &icon_url))
        }
        (components::create_server_card())
    }))
}
