use std::env;
use std::fs;
use std::path::{Path, PathBuf};

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

fn path_for_project_in(base_dir: &Path, name: &str) -> PathBuf {
    base_dir.join("servers").join(name)
}

fn create_compose_project_in(base_dir: &Path, name: &str) -> std::io::Result<()> {
    let project_dir = path_for_project_in(base_dir, name);
    fs::create_dir_all(&project_dir)?;

    let compose_path = project_dir.join("compose.yml");
    let contents = serde_yaml::to_string(&create_compose(name))
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
    fs::write(compose_path, contents)?;

    Ok(())
}

pub fn create_compose_project(name: &str) -> std::io::Result<()> {
    let base_dir = env::current_dir()?;
    create_compose_project_in(&base_dir, name)
}

pub fn list_servers() -> std::io::Result<Vec<String>> {
    let mut names = Vec::new();
    for entry in fs::read_dir("servers")? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }

        if entry.path().join("compose.yml").is_file() {
            names.push(entry.file_name().to_string_lossy().into_owned());
        }
    }

    names.sort();
    Ok(names)
}
