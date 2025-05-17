use super::{NodeKind, NodeBase, EdgeCondition};
use crate::domain::workflow::UnknownNode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Node {
    pub name: String,
    #[serde(flatten)]
    pub kind: NodeKind,
    #[serde(default)]
    pub conditions: Option<EdgeCondition>,
    #[serde(default)]
    pub next: Option<Vec<u16>>,
}

impl Node {
    pub fn get_kind(&self) -> Box<&dyn NodeBase> {
        match &self.kind {
            NodeKind::ManualTriggerV1(node) => Box::new(node),
            NodeKind::DoNothingV1(node) => Box::new(node),
            NodeKind::SetV1(node) => Box::new(node),
            NodeKind::IfV1(node) => Box::new(node),
            NodeKind::WebhookV1(node) => Box::new(node),
            NodeKind::Unknown => Box::new(&UnknownNode {}),
        }
    }
}