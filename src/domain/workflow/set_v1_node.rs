use crate::domain::entities::NodeBase;
use serde::{Deserialize, Serialize};
use serde_json::Value;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SetV1Node {
    #[serde(default)]
    pub data: Value,
}

impl NodeBase for SetV1Node {
    fn get_type(&self) -> &'static str {
        "SetV1"
    }
}