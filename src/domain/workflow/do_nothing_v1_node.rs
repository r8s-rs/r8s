use crate::domain::entities::{NodeBase, NodeKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DoNothingV1Node {
    pub node_kind: Box<NodeKind>,
}

impl NodeBase for DoNothingV1Node {
    fn get_type(&self) -> &'static str {
        "DoNothingV1"
    }

    fn is_trigger(&self) -> bool {
        false
    }
}