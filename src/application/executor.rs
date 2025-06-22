use crate::infrastructure::repositories::Workflow;
use sqlx::{Transaction, Postgres};
use std::collections::BTreeMap;
use serde_json::{Value, json};
use tera::{Tera, Context};
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
    history: BTreeMap<i64, i64>,
    memory: Value,
}

impl Executor {
    pub fn new(workflow: Workflow, initial_input: Value, execution_id: i64) -> Self {
        Self {
            workflow,
            initial_input,
            execution_id,
            history: BTreeMap::new(),
            memory: json!({
                "context": {
                    "last": {},
                },
            }),
        }
    }

    pub async fn run(&mut self, tx: &mut Transaction<'_, Postgres>, edges: &BTreeMap<i64, Edge>) -> Result<ExecutionStatus, String> {
        let mut tera = Tera::default();

        for (node_key, node) in &self.workflow.nodes {
            info!("Executando nó: [{}] - {}", node_key, node.name);

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

                    self.memory["context"][&node.name] = self.initial_input.clone();

                    if let Some(to_id) = edge.to_id {
                        self.history.insert(
                            to_id,
                            edge.from_id,
                        );
                    }
                }
                NodeKind::SetV1(set_node) => {
                    info!("SetV1: [{}]", self.execution_id);

                    let mut memory = self.get_context(
                        edge.to_id.unwrap_or(*node_key as i64),
                        edges,
                    );

                    let template = set_node.data.to_string();

                    let template = template.as_str();

                    let mut error = None::<String>;

                    match Context::from_value(memory["context"].clone()) {
                        Ok(context) => {                            
                            match tera.render_str(template, &context) {
                                Ok(rendered) => {
                                    let rendered = rendered.as_str();

                                    memory["context"][&node.name] = serde_json::from_str(rendered).unwrap();
                                }
                                Err(e) => {
                                    println!("   ➥ Erro ao renderizar: {}", e);
                                    error = Some(e.to_string());
                                    break;
                                }
                            }
                        }
                        Err(e) => {
                            println!("   ➥ Erro ao criar contexto: {}", e);
                            error = Some(e.to_string());
                            break;
                        }
                    }

                    if edge.execution_log_id.is_none() {
                        let _ = set_node.save_execution_log(
                            tx,
                            self.execution_id,
                            *node_key as i64,
                            memory["context"].get(&node.name).cloned(),
                            error
                        ).await;
                    }
                }
                NodeKind::IfV1(node) => {
                    println!("   ➥ If");
                }
                NodeKind::Unknown => println!("⚠️  Tipo desconhecido"),
            }
        }

        Ok(ExecutionStatus::Success)
    }

    fn get_context(&self, from_id: i64, edges: &BTreeMap<i64, Edge>) -> Value {
        let mut memory = self.memory.clone();

        if let Some(back_id) = self.history.get(&from_id) {
            if let Some(edge) = edges.get(&back_id) {
                info!("Recuperando contexto de: {} -> {}", edge.from_name, edge.to_name);
                memory["context"]["last"] = self.memory["context"][&edge.from_name].clone();
            }
        }

        memory
    }
}