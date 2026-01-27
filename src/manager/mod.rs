mod record;
mod worker;

use std::collections::HashMap;

use actix_web::web::Data;
use bollard::Docker;
use tokio::sync::RwLock;

pub use record::ServerRecord;

pub struct ServerManager {
    docker: Docker,
    records: RwLock<HashMap<String, ServerRecord>>,
}

impl ServerManager {
    pub fn new() -> Self {
        Self {
            docker: Docker::connect_with_defaults().expect("failed to create docker connection"),
            records: RwLock::new(HashMap::new()),
        }
    }

    pub fn spawn_background_worker(manager: Data<Self>) {
        worker::spawn(manager);
    }

    pub async fn record(&self, server_name: &str) -> Option<ServerRecord> {
        self.records.read().await.get(server_name).cloned()
    }

    pub async fn records(&self) -> HashMap<String, ServerRecord> {
        self.records.read().await.clone()
    }
}
