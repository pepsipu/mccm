use actix_web::{
    Result,
    error::ErrorInternalServerError,
    get,
    web::{Path, ServiceConfig},
};
use maud::Markup;

use crate::{components, modrinth};

#[get("/create/modrinth/{slug}")]
async fn modpack_create(slug: Path<String>) -> Result<Markup> {
    let slug = slug.into_inner();
    let project = modrinth::get_project(&slug)
        .await
        .map_err(ErrorInternalServerError)?;

    Ok(components::page(components::modpack_create_page(&project)))
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(modpack_create);
}
