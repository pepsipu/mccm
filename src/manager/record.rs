use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ServerRecord {
    pub state: String,
    pub icon_png: Option<Vec<u8>>,
    pub properties: HashMap<String, String>,
}
