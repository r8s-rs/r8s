use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct SwitchNode {
    pub key: String,
    pub cases: HashMap<String, String>,
}