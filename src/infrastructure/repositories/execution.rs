use serde::{Deserialize, Serialize};
use serde_json::Value;


#[derive(Debug, Serialize, Deserialize)]
pub struct Execution {
    pub workflow_id: i64,
    pub input: Value
}
