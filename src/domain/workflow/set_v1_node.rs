use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct SetV1Node {
    #[serde(default)]
    pub data: Value,
}
