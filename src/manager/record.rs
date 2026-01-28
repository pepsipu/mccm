#[derive(Clone, Debug)]
pub struct ServerRecord {
    container_id: String,
    state: String,
    icon_png: Option<Vec<u8>>,
    motd: Option<String>,
}

impl ServerRecord {
    pub fn new(
        container_id: String,
        state: String,
        icon_png: Option<Vec<u8>>,
        motd: Option<String>,
    ) -> Self {
        Self {
            container_id,
            state,
            icon_png,
            motd,
        }
    }

    pub fn container_id(&self) -> &str {
        &self.container_id
    }

    pub fn state(&self) -> &str {
        &self.state
    }

    pub fn icon_png(&self) -> Option<&Vec<u8>> {
        self.icon_png.as_ref()
    }

    pub fn motd(&self) -> Option<&str> {
        self.motd.as_deref()
    }
}
