use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SetV1Node {
    #[serde(default)]
    pub params: HashMap<String, serde_json::Value>,
}