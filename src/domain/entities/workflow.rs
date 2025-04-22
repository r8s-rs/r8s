use std::collections::HashMap;
use serde::Deserialize;
use super::node::Node;

#[derive(Debug, Deserialize)]
pub struct Workflow {
    pub workflow_id: String,
    pub start: String,
    pub nodes: HashMap<String, Node>,
}