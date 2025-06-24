use crate::domain::entities::{Node, NodeKind, EdgeCondition};
use crate::infrastructure::repositories::WebhookRepository;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::error;
use crate::domain::workflow::{
    ManualTriggerV1Node,
    HttpClientV1Node,
    DoNothingV1Node,
    WebhookV1Node,
    UnknownNode,
    SetV1Node,
    IfV1Node,
};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Edge {
    pub workflow_id: i64,
    pub from_id: i64,
    pub to_id: i64,
    pub from_data: Option<Value>,
    pub from_name: String,
    pub condition: Option<Value>,
    pub from_type: String,
    pub from_output: Option<Value>,
    pub from_error: Option<String>,
    pub execution_log_id: Option<i64>,
}

impl Edge {
    pub async fn get_from_node(&self, tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, last_node: Option<&Node>) -> Node {
        let data = self.from_data.clone().unwrap_or(Value::Null);

        let kind = match self.from_type.as_str() {
            "ManualTriggerV1" => NodeKind::ManualTriggerV1(ManualTriggerV1Node {}),
            "SetV1" => NodeKind::SetV1(SetV1Node {data}),
            "DoNothingV1" => NodeKind::DoNothingV1(last_node.map_or(DoNothingV1Node {
                node: Box::new(None)
            }, |n| {
                DoNothingV1Node {
                    node: Box::new(Some(n.clone()))
                }
            })),
            "IfV1" => NodeKind::IfV1(IfV1Node {data}),
            "WebhookV1" => {
                let wh = WebhookRepository::get_by_workflow_id(
                    tx,
                    self.workflow_id,
                ).await;

                NodeKind::WebhookV1(wh.unwrap().unwrap())
            }
            "HttpClientV1" => NodeKind::HttpClientV1(HttpClientV1Node {
                data: serde_json::from_value(data).unwrap()
            }),
            _ => NodeKind::Unknown,
        };

        let conditions = match self.condition {
            Some(ref condition) => {
                match serde_json::from_value::<EdgeCondition>(condition.clone()) {
                    Ok(cond) => Some(cond),
                    Err(e) => {
                        error!("Erro ao converter edge condition {e}");
                        None
                    }
                }
            },
            None => None,
        };

        Node {
            name: self.from_name.clone(),
            kind,
            conditions,
            next: Some(vec![self.to_id as u64]),
        }
    }
}