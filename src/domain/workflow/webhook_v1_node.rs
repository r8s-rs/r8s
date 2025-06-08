use crate::application::State;
use serde::{Deserialize, Serialize};
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
    pub async fn to_queue(self, _state: &State) {
        
    }
}
