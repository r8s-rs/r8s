use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EdgeCondition {
    #[serde(rename = "left")]
    left: Option<String>,
    #[serde(rename = "op")]
    op: Option<String>,
    #[serde(rename = "right")]
    right: Option<Value>,
    #[serde(rename = "and")]
    and: Option<Vec<EdgeCondition>>,
    #[serde(rename = "or")]
    or: Option<Vec<EdgeCondition>>,
}