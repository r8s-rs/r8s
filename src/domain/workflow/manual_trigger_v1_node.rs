use crate::domain::entities::NodeBase;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ManualTriggerV1Node;

impl NodeBase for ManualTriggerV1Node {
    fn get_type(&self) -> &'static str {
        "ManualTriggerV1"
    }

    fn is_trigger(&self) -> bool {
        true
    }
}