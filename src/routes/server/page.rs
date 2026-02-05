use crate::{components, compose};
use actix_web::{
    HttpResponse, Result,
    error::{ErrorInternalServerError, ErrorNotFound},
    get,
    http::header,
    post,
    web::Path,
};
use docker_compose_types::{Environment, SingleValue};
use indexmap::IndexMap;
use maud::{Markup, html};
use serde::Deserialize;
use serde_qs::actix::QsForm;

fn env_pairs(env: &Environment) -> Vec<(String, String)> {
    match env {
        Environment::KvPair(map) => map
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    v.as_ref().map(ToString::to_string).unwrap_or_default(),
                )
            })
            .collect(),
        Environment::List(list) => list
            .iter()
            .map(|item| match item.split_once('=') {
                Some((k, v)) => (k.to_string(), v.to_string()),
                None => (item.to_string(), String::new()),
            })
            .collect(),
    }
}

#[derive(Debug, Deserialize)]
struct EnvForm {
    key: Vec<String>,
    value: Vec<String>,
}

#[get("/{server_name:[A-Za-z0-9_-]+}")]
async fn server_page(server_name: Path<String>) -> Result<Markup> {
    let server_name = server_name.into_inner();
    let compose = compose::read_compose_project(&server_name).map_err(ErrorInternalServerError)?;
    let env = compose
        .services
        .0
        .get("mc")
        .and_then(|service| service.as_ref())
        .map(|service| &service.environment)
        .ok_or_else(|| ErrorNotFound("mc service not found"))?;
    let env = env_pairs(env);
    let action = format!("/server/{server_name}");
    Ok(components::page(html! {
        h2 { (server_name) }
        form method="post" action=(action) {
            @for (key, value) in env {
                div {
                    input type="text" name="key[]" value=(key) {} textarea name="value[]" { (value) }
                }
            }
            div { input type="text" name="key[]" {} textarea name="value[]" {} } button type="submit" { "Save" }
        }
    }))
}
#[post("/{server_name:[A-Za-z0-9_-]+}")]

async fn save_server_page(
    server_name: Path<String>,
    form: QsForm<EnvForm>,
) -> Result<HttpResponse> {
    let server_name = server_name.into_inner();
    let mut compose =
        compose::read_compose_project(&server_name).map_err(ErrorInternalServerError)?;
    let service = compose
        .services
        .0
        .get_mut("mc")
        .and_then(|service| service.as_mut())
        .ok_or_else(|| ErrorNotFound("mc service not found"))?;
    let mut env = IndexMap::new();
    for (key, value) in form.key.iter().zip(form.value.iter()) {
        let key = key.trim();
        if key.is_empty() {
            continue;
        }
        env.insert(
            key.to_string(),
            Some(SingleValue::String(value.to_string())),
        );
    }
    service.environment = Environment::KvPair(env);
    compose::write_compose_project(&server_name, &compose).map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::SeeOther()
        .insert_header((header::LOCATION, format!("/server/{server_name}")))
        .finish())
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(server_page).service(save_server_page);
}
