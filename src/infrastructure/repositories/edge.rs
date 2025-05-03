use serde_json::Value;

pub struct Edge {
    pub from_node_id: Option<i64>,
    pub to_node_id: Option<i64>,
    pub condition: Value,
}
