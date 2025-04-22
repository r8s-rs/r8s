use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SetNode {
    #[serde(default)]
    pub params: HashMap<String, serde_json::Value>,
}