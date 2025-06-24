use crate::{application::TemplateRender, infrastructure::repositories::ExecutionLogRepository};
use crate::domain::entities::NodeBase;
use serde::{Deserialize, Serialize};
use sqlx::{Transaction, Postgres};
use std::sync::{Arc, Mutex};
use serde_json::Value;


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SetV1Node {
    #[serde(default)]
    pub data: Value,
}

impl NodeBase for SetV1Node {
    fn get_type(&self) -> &'static str {
        "SetV1"
    }

    fn is_trigger(&self) -> bool {
        false
    }
}

impl SetV1Node {
    pub async fn save_execution_log(&self, tx: &mut Transaction<'_, Postgres>, execution_id: i64, node_id: i64, output: Option<Value>, error: Option<String>) -> Result<(), sqlx::Error> {
        ExecutionLogRepository::insert(tx, execution_id, node_id, output, error).await
    }

    pub async fn execute(&self, tx: &mut Transaction<'_, Postgres>, execution_id: i64, execution_log_id: Option<i64>, node_id: i64, local_memory: &mut Value, template_render: Arc<Mutex<TemplateRender>>, node_name: &str, error: &mut Option<String>, memory: &mut Value) -> Option<Value> {
        if let Some(output) = local_memory["context"].get(node_name) {
            println!("   ➥ SetV1: output ja existe");
            return Some(output.clone());
        }

        let template = self.data.to_string();

        let template = template.as_str();

        let mut template_render = template_render.lock().unwrap();

        match template_render.render_str(template, local_memory["context"].clone()) {
            Ok(rendered) => {
                local_memory["context"][node_name] = serde_json::from_str(rendered.as_str()).unwrap();
                memory["context"][node_name] = local_memory["context"][node_name].clone();
            }
            Err(e) => {
                println!("   ➥ Erro ao renderizar SetV1: {}", e);
                *error = Some(e.to_string());
                memory["context_errors"][node_name] = e.to_string().into();
            }
            
        }

        let output = local_memory["context"].get(node_name).cloned();

        if execution_log_id.is_none() {
            let _ = self.save_execution_log(
                tx,
                execution_id,
                node_id,
                output.clone(),
                error.clone(),
            ).await;
        }

        output
    }
}