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

#[derive(Debug, Deserialize)]
struct EnvForm {
    key: Vec<String>,
    value: Vec<String>,
}

#[get("/{server_name}")]
async fn server_page(docker: Data<Docker>, server_name: Path<String>) -> Result<Markup> {
    let server_name = server_name.into_inner();
    let compose = compose::read_compose_project(&server_name).map_err(ErrorInternalServerError)?;
    let service =
        compose::mc_service(&compose).ok_or_else(|| ErrorNotFound("mc service not found"))?;
    let env = server::env_pairs(&service.environment);

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
    let service = compose::mc_service_mut(&mut compose)
        .ok_or_else(|| ErrorNotFound("mc service not found"))?;
    let mut env = IndexMap::new();
    for (key, value) in form.key.iter().zip(form.value.iter()) {
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
