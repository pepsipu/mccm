use docker_compose_types::Environment;
use indexmap::IndexMap;

use crate::compose;

pub fn read_mc_env(server_name: &str) -> anyhow::Result<IndexMap<String, String>> {
    let compose = compose::read_compose_project(server_name)?;
    let service =
        compose::mc_service(&compose).ok_or_else(|| anyhow::anyhow!("mc service not found"))?;
    Ok(env_to_map(&service.environment))
}

pub fn read_mc_env_value(server_name: &str, key: &str) -> Option<String> {
    read_mc_env(server_name)
        .ok()
        .and_then(|env| env.get(key).cloned())
}

pub fn env_pairs(env: &Environment) -> Vec<(String, String)> {
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

fn env_to_map(env: &Environment) -> IndexMap<String, String> {
    env_pairs(env).into_iter().collect()
}
