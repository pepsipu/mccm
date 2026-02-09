use actix_web::{HttpResponse, Result, error::ErrorInternalServerError, http::header, web};
use docker_compose_types::{Environment, SingleValue};
use indexmap::IndexMap;
use maud::Markup;
use serde::Deserialize;
use serde_qs::actix::QsForm;

use crate::{components, compose, modrinth};

#[derive(Deserialize)]
struct SearchQuery {
    q: Option<String>,
    error: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CreateRequest {
    name: String,
    #[serde(default)]
    key: Vec<String>,
    #[serde(default)]
    value: Vec<String>,
}

async fn get_create(query: web::Query<SearchQuery>) -> Result<Markup> {
    let (modrinth_error, modpacks) = match modrinth::search_modpacks(query.q.as_deref()).await {
        Ok(v) => (false, v),
        Err(_) => (true, vec![]),
    };

    Ok(components::page(components::create_page(
        query.q.as_deref(),
        query.error.as_deref(),
        modrinth_error,
        &modpacks,
    )))
}

async fn post_create(form: QsForm<CreateRequest>) -> Result<HttpResponse> {
    let name = form.name.trim();
    if name.is_empty() {
        return Ok(HttpResponse::SeeOther()
            .insert_header((header::LOCATION, "/create?error=invalid"))
            .finish());
    }

    match compose::server_exists(name) {
        Ok(true) => {
            return Ok(HttpResponse::SeeOther()
                .insert_header((header::LOCATION, "/create?error=exists"))
                .finish());
        }
        Ok(false) => {}
        Err(_) => {
            return Ok(HttpResponse::SeeOther()
                .insert_header((header::LOCATION, "/create?error=invalid"))
                .finish());
        }
    }

    compose::create_compose_project(name).map_err(ErrorInternalServerError)?;

    if !form.key.is_empty() || !form.value.is_empty() {
        let mut compose = compose::read_compose_project(name).map_err(ErrorInternalServerError)?;
        let service = compose::mc_service_mut(&mut compose)
            .ok_or_else(|| ErrorInternalServerError("mc service not found"))?;

        let mut env: IndexMap<String, Option<SingleValue>> = IndexMap::new();
        for (k, v) in form.key.iter().zip(form.value.iter()) {
            if k.trim().is_empty() {
                continue;
            }
            env.insert(k.to_string(), Some(SingleValue::String(v.to_string())));
        }
        env.entry("EULA".to_string())
            .or_insert_with(|| Some(SingleValue::String("TRUE".to_string())));

        service.environment = Environment::KvPair(env);
        compose::write_compose_project(name, &compose).map_err(ErrorInternalServerError)?;
    }

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
