use crate::{application::State, domain::entities::Workflow};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Mutex;
pub use request::Request;
mod request;


#[derive(Debug, Deserialize, Serialize)]
pub struct WebhookV1Node {
    pub request: Request,
    pub next: Vec<String>,
}

impl WebhookV1Node {
    pub async fn to_queue(self, state: &State) {
        
    }
}
