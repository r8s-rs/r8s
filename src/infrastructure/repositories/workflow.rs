use serde::{Deserialize, Serialize};
use crate::domain::entities::Node;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Workflow {
    pub pub_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub nodes: HashMap<String, Node>,
}