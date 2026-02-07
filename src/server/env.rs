use docker_compose_types::Environment;
use indexmap::IndexMap;

use crate::compose;

pub fn mc_env(server_name: &str) -> anyhow::Result<IndexMap<String, String>> {
    let compose = compose::read_compose_project(server_name)?;
    let service = compose
        .services
        .0
        .get("mc")
        .and_then(|svc| svc.as_ref())
        .ok_or_else(|| anyhow::anyhow!("mc service not found"))?;
    Ok(env_to_map(&service.environment))
}

fn env_to_map(env: &Environment) -> IndexMap<String, String> {
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
