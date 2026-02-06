use std::path::PathBuf;
use std::{env, fs};

use anyhow::Result;
use docker_compose_types::{Compose, Environment, Service, Services, SingleValue};
use indexmap::indexmap;

fn create_compose(name: &str) -> Compose {
    Compose {
        name: Some(name.to_string()),
        services: Services(indexmap! {
            "mc".to_string() => Some(Service {
                image: Some("itzg/minecraft-server".to_string()),
                environment: Environment::KvPair(indexmap! {
                    "EULA".to_string() => Some(SingleValue::String("TRUE".to_string()))
                }),
                ..Default::default()
            }),
        }),
        ..Default::default()
    }
}

fn join_segment(mut base: PathBuf, seg: &str) -> Result<PathBuf> {
    use std::path::{Component, Path};

    let mut it = Path::new(seg).components();
    match (it.next(), it.next()) {
        (Some(Component::Normal(_)), None) => {
            base.push(seg);
            Ok(base)
        }
        _ => anyhow::bail!("invalid path segment"),
    }
}

fn compose_path(server_name: &str) -> Result<PathBuf> {
    Ok(join_segment(env::current_dir()?.join("servers"), server_name)?.join("compose.yml"))
}

pub fn create_compose_project(name: &str) -> Result<()> {
    let path = compose_path(name)?;
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(path, serde_yaml::to_string(&create_compose(name))?)?;
    Ok(())
}

pub fn read_compose_project(server_name: &str) -> Result<Compose> {
    let contents = fs::read_to_string(compose_path(server_name)?)?;
    Ok(serde_yaml::from_str(&contents)?)
}

pub fn write_compose_project(server_name: &str, compose: &Compose) -> Result<()> {
    fs::write(compose_path(server_name)?, serde_yaml::to_string(compose)?)?;
    Ok(())
}

pub fn list_servers() -> Result<Vec<String>> {
    Ok(match fs::read_dir("servers") {
        Ok(entries) => entries
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect(),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => vec![],
        Err(e) => return Err(e.into()),
    })
}
