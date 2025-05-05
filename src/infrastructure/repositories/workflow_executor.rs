use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use crate::domain::entities::Node;
use std::collections::BTreeMap;


#[derive(Deserialize, Serialize)]
pub struct WorkflowExecutor {
    pub id: String,
    pub nodes: BTreeMap<String, Node>
}

impl WorkflowExecutor {
    pub async fn to_queue(&self, _pool: &PgPool) -> Result<Vec<i64>, sqlx::Error> {
        /*
        let mut tx = pool.begin().await?;
        
        for node in self.nodes {
            sqlx::query!("
            INSERT INTO execution (
                previous_execution_id,
                workflow_id,
                status,
                node_key
                ) VALUES (
                    $1,
                    $2,
                    $3,
                    $4
                )
                ",
                tx
                );
            }
            
        */
        Ok(vec![])
    }
}