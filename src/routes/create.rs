use actix_web::{Result, get};
use maud::{Markup, html};

use crate::components;

#[get("/create")]
pub async fn create() -> Result<Markup> {
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
