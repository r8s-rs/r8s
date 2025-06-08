use crate::domain::entities::NodeBase;
use serde::{Deserialize, Serialize};
use serde_json::Value;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IfV1Node {
    #[serde(default)]
    pub data: Value,
}

impl NodeBase for IfV1Node {
    fn get_type(&self) -> &'static str {
        "IfV1"
    }

    fn is_trigger(&self) -> bool {
        false
    }
}