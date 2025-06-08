use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Execution {
    pub id: i64,
    pub workflow_id: i64,
    pub input: Value
}