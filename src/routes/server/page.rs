use crate::{components, compose, server};
use actix_web::{
    HttpResponse, Result,
    error::{ErrorInternalServerError, ErrorNotFound},
    get,
    http::header,
    post,
    web::{Data, Path},
};
use bollard::Docker;
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

#[get("/{server_name}")]
async fn server_page(docker: Data<Docker>, server_name: Path<String>) -> Result<Markup> {
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

    let state = server::get_server(&docker, &server_name)
        .await
        .map_err(ErrorInternalServerError)?
        .map(|s| s.state)
        .unwrap_or_default();

    Ok(components::page(html! {
        (components::server_card(&server_name, &state))
        (components::env_editor(&server_name, &env))
    }))
}
#[post("/{server_name}")]
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
