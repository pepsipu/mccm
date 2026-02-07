mod env;
mod list;

pub const MINECRAFT_SERVER_IMAGE: &str = "itzg/minecraft-server";
pub const PROJECT_LABEL_KEY: &str = "com.docker.compose.project";

pub use env::read_mc_env;
pub use list::{get_server, get_servers};
