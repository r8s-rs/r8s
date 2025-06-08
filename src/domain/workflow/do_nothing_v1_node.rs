use crate::domain::entities::{NodeBase, Node};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DoNothingV1Node {
    pub node: Box<Option<Node>>,
}

impl NodeBase for DoNothingV1Node {
    fn get_type(&self) -> &'static str {
        "DoNothingV1"
    }

    fn is_trigger(&self) -> bool {
        false
    }
}