use crate::infrastructure::repositories::ExecutionLogRepository;
use crate::domain::entities::NodeBase;
use serde::{Deserialize, Serialize};
use sqlx::{Transaction, Postgres};
use tera::{Tera, Context};
use serde_json::Value;

mod request;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HttpClientV1Node {
    pub data: request::Request,
}

impl NodeBase for HttpClientV1Node {
    fn get_type(&self) -> &'static str {
        "HttpClientV1"
    }

    fn is_trigger(&self) -> bool {
        false
    }
}

impl HttpClientV1Node {
    pub async fn save_execution_log(&self, tx: &mut Transaction<'_, Postgres>, execution_id: i64, node_id: i64, output: Option<Value>, error: Option<String>) -> Result<(), sqlx::Error> {
        ExecutionLogRepository::insert(tx, execution_id, node_id, output, error).await
    }

    pub async fn execute(&self, tx: &mut Transaction<'_, Postgres>, execution_id: i64, execution_log_id: Option<i64>, node_id: i64, local_memory: &mut Value, tera: &mut Tera, node_name: &str, error: &mut Option<String>, memory: &mut Value) -> Option<Value> {
        if let Some(output) = local_memory["context"].get(node_name) {
            return Some(output.clone());
        }

        /*let template = self.data.to_string();

        let template = template.as_str();

        match Context::from_value(local_memory["context"].clone()) {
            Ok(context) => {                            
                match tera.render_str(template, &context) {
                    Ok(rendered) => {
                        let rendered = rendered.as_str();

                        local_memory["context"][node_name] = serde_json::from_str(rendered).unwrap();
                        memory["context"][node_name] = local_memory["context"][node_name].clone();
                    }
                    Err(e) => {
                        println!("   ➥ Erro ao renderizar: {}", e);
                        *error = Some(e.to_string());
                        memory["context_errors"][node_name] = e.to_string().into();
                    }
                }
            }
            Err(e) => {
                println!("   ➥ Erro ao criar contexto: {}", e);
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

        output*/

        None
    }
}