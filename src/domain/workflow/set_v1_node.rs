use crate::infrastructure::repositories::ExecutionLogRepository;
use crate::domain::entities::NodeBase;
use serde::{Deserialize, Serialize};
use sqlx::{Transaction, Postgres};
use serde_json::Value;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SetV1Node {
    #[serde(default)]
    pub data: Value,
}

impl NodeBase for SetV1Node {
    fn get_type(&self) -> &'static str {
        "SetV1"
    }

    fn is_trigger(&self) -> bool {
        false
    }
}

impl SetV1Node {
    pub async fn save_execution_log(&self, tx: &mut Transaction<'_, Postgres>, execution_id: i64, node_id: i64, output: Option<Value>, error: Option<String>) -> Result<(), sqlx::Error> {
        ExecutionLogRepository::insert(tx, execution_id, node_id, output, error).await
    }
}