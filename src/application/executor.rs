use crate::{application::TemplateRender, infrastructure::repositories::Workflow};
use std::{collections::BTreeMap, sync::{Arc, Mutex}};
use sqlx::{Transaction, Postgres};
use serde_json::{Value, json};
use tracing::info;
use crate::domain::entities::{
    ExecutionStatus,
    NodeKind,
    Edge,
};

pub struct Executor {
    pub workflow: Workflow,
    pub template_render: Arc<Mutex<TemplateRender>>,
    pub initial_input: Value,
    pub execution_id: i64,
    history: BTreeMap<i64, i64>,
    memory: Value,
}

impl Executor {
    pub fn new(workflow: Workflow, initial_input: Value, execution_id: i64, template_render: Arc<Mutex<TemplateRender>>) -> Self {
        Self {
            workflow,
            initial_input,
            execution_id,
            history: BTreeMap::new(),
            template_render,
            memory: json!({
                "context": {
                    "last": {},
                },
                "context_errors": {}
            }),
        }
    }

    pub async fn run(&mut self, tx: &mut Transaction<'_, Postgres>, edges: &BTreeMap<i64, Edge>) -> Result<ExecutionStatus, String> {
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

                    self.history.entry(edge.to_id).or_insert(edge.from_id);
                }
                NodeKind::HttpClientV1(http_client_node) => {
                    info!("HttpClientV1: [{}]", self.execution_id);
                    let mut memory = self.get_context(
                        edge.to_id,
                        edges,
                    );

                    dbg!(memory);

                    //dbg!(&self.memory);
                    /*
                    let mut memory = self.get_context(
                        edge.to_id.unwrap_or(*node_key as i64),
                        edges,
                    );
                    
                    dbg!(&memory);
                    
                    let mut error = None::<String>;
                    
                    let _ = http_client_node.execute(
                        tx,
                        self.execution_id,
                        edge.execution_log_id,
                        *node_key as i64,
                        &mut memory,
                        &mut tera,
                        &node.name,
                        &mut error,
                        &mut self.memory,
                    );
                    */
                }
                NodeKind::SetV1(set_node) => {
                    info!("SetV1: [{}]", self.execution_id);

                    dbg!(edge.to_id, node_key);

                    let mut memory = self.get_context(
                        edge.from_id,
                        edges,
                    );

                    //dbg!(&memory);

                    let mut error = None::<String>;

                    let _ = set_node.execute(
                        tx,
                        self.execution_id,
                        edge.execution_log_id,
                        *node_key as i64,
                        &mut memory,
                        self.template_render.clone(),
                        &node.name,
                        &mut error,
                        &mut self.memory,
                    ).await;

                    //dbg!(&self.memory);

                    self.history.entry(edge.to_id).or_insert(edge.from_id);
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
                info!("Recuperando contexto de: {}", edge.from_name);
                match self.memory["context"][&edge.from_name].clone() {
                    Value::Null => {
                        info!("Contexto não encontrado para: {}", edge.from_name);
                        memory["context"]["last"] = json!({});
                    }
                    value => {
                        info!("Contexto encontrado: {}", value);
                        memory["context"]["last"] = value;
                    }   
                }
            }
        }

        memory
    }
}