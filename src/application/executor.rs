use crate::infrastructure::repositories::Workflow;
use sqlx::{Transaction, Postgres};
use std::collections::BTreeMap;
use serde_json::Value;
use tracing::info;
use crate::domain::entities::{
    ExecutionStatus,
    NodeKind,
    Edge,
};

pub struct Executor {
    pub workflow: Workflow,
    pub initial_input: Value,
    pub execution_id: i64,
    memory: BTreeMap<u64, Value>,
}

impl Executor {
    pub fn new(workflow: Workflow, initial_input: Value, execution_id: i64) -> Self {
        Self {
            workflow,
            initial_input,
            execution_id,
            memory: BTreeMap::new(),
        }
    }

    pub async fn run(&mut self, tx: &mut Transaction<'_, Postgres>, edges: &BTreeMap<i64, Edge>) -> Result<ExecutionStatus, String> {
        for (node_key, node) in &self.workflow.nodes {
            let edge = edges.get(&(*node_key as i64)).unwrap();

            match &node.kind {
                NodeKind::ManualTriggerV1(_) => {
                    println!("   ➥ Manual Trigger");
                }
                NodeKind::DoNothingV1(node) => {
                    println!("   ➥ Do nothing");
                }
                NodeKind::WebhookV1(webhook_node) => {
                    info!("WebhookV1: [{}]", self.execution_id);

                    if edge.execution_log_id.is_none() {
                        let _ = webhook_node.save_execution_log(
                            tx,
                            self.execution_id,
                            *node_key as i64,
                            Some(self.initial_input.clone()),
                            None
                        ).await;
                    }

                    self.memory.insert(*node_key as u64, self.initial_input.clone());
                }
                NodeKind::SetV1(set_node) => {
                    println!("   ➥ Set vars: {:?}", set_node.data);
                }
                NodeKind::IfV1(node) => {
                    println!("   ➥ If");
                }
                NodeKind::Unknown => println!("⚠️  Tipo desconhecido"),
            }
        }

        Ok(ExecutionStatus::Success)
    }
}