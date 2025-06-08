use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Debug, Deserialize, Serialize, Clone, FromRow)]
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