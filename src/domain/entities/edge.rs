use crate::domain::entities::{Node, NodeKind, EdgeCondition};
use crate::infrastructure::repositories::WebhookRepository;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::error;
use crate::domain::workflow::{
    ManualTriggerV1Node,
    DoNothingV1Node,
    WebhookV1Node,
    UnknownNode,
    SetV1Node,
    IfV1Node,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Edge {
    pub workflow_id: i64,
    pub from_id: i64,
    pub to_ids: Option<Vec<i64>>,
    pub from_data: Option<Value>,
    pub condition: Value,
    pub from_type: String,
}

impl Edge {
    pub async fn get_from_node(&self, tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, last_node: Option<&Node>) -> Node {
        let data = self.from_data.clone().unwrap();

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
            _ => NodeKind::Unknown,
        };

        let conditions = match serde_json::from_value::<EdgeCondition>(self.condition.clone()) {
            Ok(condition) => Some(condition),
            Err(e) => {
                match self.condition {
                    Value::Null => None,
                    _ => {
                        error!("Erro ao converter edge condition {e}",);
                        None
                    }
                }
            }
        };

        let next = self.to_ids.as_ref().map(|vec| vec.into_iter().map(|x| *x as u64).collect());

        Node {
            name: "".to_string(),
            kind,
            conditions,
            next,
        }
    }
}