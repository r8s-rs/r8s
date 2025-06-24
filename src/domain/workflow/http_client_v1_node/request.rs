use crate::domain::entities::HttpMethod;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Request {
    pub url: String,
    pub method: HttpMethod,
    #[serde(default)]
    pub headers: Option<HashMap<String, String>>,
    #[serde(default)]
    pub query_params: Option<HashMap<String, String>>,
    #[serde(default)]
    pub form_data: Option<Value>,
}