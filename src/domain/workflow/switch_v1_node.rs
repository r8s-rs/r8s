use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct SwitchV1Node {
    pub key: String,
    pub cases: HashMap<String, String>,
}