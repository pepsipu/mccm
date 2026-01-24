use std::env;
use std::fs::{self, OpenOptions};
use std::path::PathBuf;

pub fn path_for_project(name: &str) -> PathBuf {
    env::current_dir().unwrap().join("servers").join(name)
}

pub fn create_compose_project(name: &str) -> std::io::Result<()> {
    let project_dir = path_for_project(name);
    fs::create_dir_all(&project_dir)?;

    let compose_path = project_dir.join("compose.yml");
    OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(compose_path)?;

    Ok(())
}
