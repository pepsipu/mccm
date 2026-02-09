use actix_web::{HttpResponse, Result, error::ErrorInternalServerError, http::header, web};
use docker_compose_types::{Environment, SingleValue};
use indexmap::IndexMap;
use maud::{Markup, html};
use serde::Deserialize;

use crate::{components, compose, modrinth};

#[derive(Deserialize)]
struct CreateForm {
    name: String,
}

#[derive(Deserialize)]
struct SearchQuery {
    q: Option<String>,
    error: Option<String>,
}

#[derive(Deserialize)]
struct CreateModrinthForm {
    name: String,
    modpack: String,
    title: String,
    icon_url: Option<String>,
}

async fn get_create(query: web::Query<SearchQuery>) -> Result<Markup> {
    let modpacks = match modrinth::search_modpacks(query.q.as_deref()).await {
        Ok(v) => v,
        Err(_) => vec![],
    };

    Ok(components::page(html! {
        h2 { "Create server" }

        @if query.error.as_deref() == Some("exists") {
            p { "That server name already exists." }
        }

        form method="post" action="/create" {
            label for="name" { "Name" }
            input type="text" name="name" id="name" {}
            button type="submit" { "Create blank" }
        }

        h2 { "Create from Modrinth modpack" }
        form method="get" action="/create" {
            label for="q" { "Search" }
            input type="text" name="q" id="q" value=(query.q.as_deref().unwrap_or("")) {}
            button type="submit" { "Search" }
        }

        @if modpacks.is_empty() {
            p { "No modpacks found." }
        }

        @for pack in modpacks {
            div .card {
                @if let Some(icon_url) = &pack.icon_url {
                    img src=(icon_url) width="64px" height="64px" {}
                }
                div .card-body {
                    strong { (pack.title) }
                    div { (pack.description) }
                    div {
                        small { (format!("Downloads: {}  Follows: {}  Updated: {}", pack.downloads, pack.follows, pack.date_modified)) }
                    }

                    form method="post" action="/create/modrinth" {
                        input type="hidden" name="modpack" value=(pack.slug) {}
                        input type="hidden" name="title" value=(pack.title) {}
                        @if let Some(icon_url) = &pack.icon_url {
                            input type="hidden" name="icon_url" value=(icon_url) {}
                        }

                        label for=(format!("name-{}", pack.project_id)) { "Server name" }
                        input type="text" name="name" id=(format!("name-{}", pack.project_id)) value=(pack.slug) {}

                        button type="submit" { "Create" }
                    }
                }
            }
        }
    }))
}

async fn post_create(form: web::Form<CreateForm>) -> Result<HttpResponse> {
    if compose::server_exists(&form.name).map_err(ErrorInternalServerError)? {
        return Ok(HttpResponse::SeeOther()
            .insert_header((header::LOCATION, "/create?error=exists"))
            .finish());
    }
    compose::create_compose_project(&form.name).map_err(ErrorInternalServerError)?;
    Ok(HttpResponse::SeeOther()
        .insert_header((header::LOCATION, "/"))
        .finish())
}

async fn post_create_modrinth(form: web::Form<CreateModrinthForm>) -> Result<HttpResponse> {
    if compose::server_exists(&form.name).map_err(ErrorInternalServerError)? {
        return Ok(HttpResponse::SeeOther()
            .insert_header((header::LOCATION, "/create?error=exists"))
            .finish());
    }
    compose::create_compose_project(&form.name).map_err(ErrorInternalServerError)?;

    let mut compose =
        compose::read_compose_project(&form.name).map_err(ErrorInternalServerError)?;
    let service = compose::mc_service_mut(&mut compose)
        .ok_or_else(|| ErrorInternalServerError("mc service not found"))?;

    let mut env: IndexMap<String, Option<SingleValue>> = IndexMap::new();
    env.insert(
        "EULA".to_string(),
        Some(SingleValue::String("TRUE".to_string())),
    );
    env.insert(
        "TYPE".to_string(),
        Some(SingleValue::String("MODRINTH".to_string())),
    );
    env.insert(
        "MODRINTH_MODPACK".to_string(),
        Some(SingleValue::String(form.modpack.to_string())),
    );
    env.insert(
        "MOTD".to_string(),
        Some(SingleValue::String(form.title.to_string())),
    );
    if let Some(icon_url) = &form.icon_url {
        env.insert(
            "ICON".to_string(),
            Some(SingleValue::String(icon_url.to_string())),
        );
    }
    service.environment = Environment::KvPair(env);

    compose::write_compose_project(&form.name, &compose).map_err(ErrorInternalServerError)?;

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
    cfg.service(web::resource("/create/modrinth").route(web::post().to(post_create_modrinth)));
}
