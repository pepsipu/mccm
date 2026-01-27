#[derive(Clone, Debug)]
pub struct ServerRecord {
    container_id: String,
    state: String,
    icon_png: Option<Vec<u8>>,
}

impl ServerRecord {
    pub fn new(container_id: String, state: String, icon_png: Option<Vec<u8>>) -> Self {
        Self {
            container_id,
            state,
            icon_png,
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
}
