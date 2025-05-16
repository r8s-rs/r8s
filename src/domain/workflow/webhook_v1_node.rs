use crate::application::State;
use serde::{Deserialize, Serialize};
mod request;
use crate::domain::entities::{NodeBase, HttpMethod};


#[derive(Debug, Deserialize, Serialize)]
pub struct WebhookV1Node {
    pub path: String,
    pub method: HttpMethod,
    pub response_code: i64,
    pub next: Option<Vec<String>>,
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
