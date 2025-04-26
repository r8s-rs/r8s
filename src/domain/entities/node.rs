use super::node_kind::NodeKind;
use std::collections::HashMap;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Node {
    pub name: String,
    #[serde(flatten)]
    pub kind: NodeKind,
    #[serde(default)]
    pub next: Vec<String>,
}