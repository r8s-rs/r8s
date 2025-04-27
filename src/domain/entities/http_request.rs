use std::{collections::HashMap, net::IpAddr};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct HttpRequest {
    pub ip: Option<IpAddr>,
    pub path: String,
    pub method: String,
    pub headers: HashMap<String, String>,
    pub query_params: Value,
    pub form_data: Value,
    pub body: Value,
}
