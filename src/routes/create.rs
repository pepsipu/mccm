use actix_web::{HttpResponse, Result, error::ErrorInternalServerError, http::header, web};
use maud::{Markup, html};
use serde::Deserialize;

use crate::{components, compose};

#[derive(Deserialize)]
struct CreateForm {
    name: String,
}

async fn get_create() -> Result<Markup> {
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

async fn post_create(form: web::Form<CreateForm>) -> Result<HttpResponse> {
    compose::create_compose_project(&form.name).map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::SeeOther()
        .insert_header((header::LOCATION, "/"))
        .finish())
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/create")
            .route(web::get().to(get_create))
            .route(web::post().to(post_create)),
    );
}
