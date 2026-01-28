use std::collections::HashMap;

use actix_web::web::Data;
use bollard::{Docker, query_parameters::EventsOptionsBuilder};
use futures_util::StreamExt;

use crate::manager::{ServerManager, ServerRecord};
use crate::server::{MINECRAFT_SERVER_IMAGE, PROJECT_LABEL_KEY, ServerStateError};

pub fn spawn(manager: Data<ServerManager>) {
    actix_web::rt::spawn(async move { run(manager).await });
}

async fn run(manager: Data<ServerManager>) {
    let docker = &manager.docker;

    let mut filters = HashMap::new();
    filters.insert("type", vec!["container".to_string()]);
    filters.insert("image", vec![MINECRAFT_SERVER_IMAGE.to_string()]);
    let options = EventsOptionsBuilder::new().filters(&filters).build();

    refresh_logged(docker, &manager).await;

    loop {
        let mut events = docker.events(Some(options.clone()));
        while let Some(next) = events.next().await {
            match next {
                Ok(_) => refresh_logged(&docker, &manager).await,
                Err(err) => {
                    eprintln!("[mccm] docker events error: {err}");
                    break;
                }
            }
        }
    }
}

async fn refresh_logged(docker: &Docker, manager: &ServerManager) {
    if let Err(err) = refresh_state(docker, manager).await {
        eprintln!("[mccm] docker refresh error: {err}");
    }
}

async fn refresh_state(docker: &Docker, manager: &ServerManager) -> Result<(), ServerStateError> {
    let containers = crate::server::list_servers(docker).await?;

    let mut next_state = HashMap::new();
    for crate::server::ServerSummary {
        project,
        id: container_id,
        state,
    } in containers
    {
        let icon_png = match crate::server::download_server_icon(docker, &container_id).await {
            Ok(Some(png)) => Some(png),
            Ok(None) => None,
            Err(err) => {
                eprintln!(
                    "[mccm] icon refresh failed: {} ({}={})",
                    err, PROJECT_LABEL_KEY, project
                );
                None
            }
        };

        let motd = match crate::server::download_server_motd(docker, &container_id).await {
            Ok(motd) => motd,
            Err(err) => {
                eprintln!(
                    "[mccm] motd refresh failed: {} ({}={})",
                    err, PROJECT_LABEL_KEY, project
                );
                None
            }
        };

        next_state.insert(project, ServerRecord::new(container_id, state, icon_png, motd));
    }

    *manager.records.write().await = next_state;
    Ok(())
}
