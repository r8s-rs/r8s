use crate::infrastructure::repositories::ExecutionLogRepository;
use sqlx::{Transaction, Postgres, Error};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
mod request;
use crate::domain::entities::{
    NodeBase,
    HttpMethod,
};


#[derive(Debug, Deserialize, Serialize, FromRow, Clone)]
pub struct WebhookV1Node {
    pub path: String,
    pub method: HttpMethod,
    pub response_code: i16,
}

impl NodeBase for WebhookV1Node {
    fn get_type(&self) -> &'static str {
        "WebhookV1"
    }

    fn is_trigger(&self) -> bool {
        true
    }
}

impl WebhookV1Node {
    pub async fn save_execution_log(&self, tx: &mut Transaction<'_, Postgres>, execution_id: i64, node_id: i64, output: Option<Value>, error: Option<String>) -> Result<(), Error> {
        ExecutionLogRepository::insert(tx, execution_id, node_id, output, error).await
    }
}
