use std::{collections::HashMap, net::IpAddr};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct HttpRequest {
    pub ip: Option<IpAddr>,
    pub path: String,
    pub method: String,
    pub headers: Option<HashMap<String, String>>,
    pub query_params: Option<Value>,
    pub form_data: Option<Value>,
    pub body: Option<Value>,
}
