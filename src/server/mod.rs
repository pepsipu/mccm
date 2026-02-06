mod file;
mod icon;
mod list;
mod properties;

pub const MINECRAFT_SERVER_IMAGE: &str = "itzg/minecraft-server";
pub const PROJECT_LABEL_KEY: &str = "com.docker.compose.project";

pub use icon::download_server_icon;
pub use list::{get_server, get_servers};
pub use properties::download_server_properties;
