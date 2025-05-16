use crate::domain::entities::NodeBase;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UnknownNode;

impl NodeBase for UnknownNode {
    fn get_type(&self) -> &'static str {
        "Unknown"
    }

    fn is_trigger(&self) -> bool {
        false
    }
}