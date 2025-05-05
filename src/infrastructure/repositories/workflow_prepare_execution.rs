use serde::{Deserialize, Serialize};
use serde_json::{Value, from_value};
use crate::domain::entities::Node;
use std::collections::BTreeMap;
use super::WorkflowExecutor;


#[derive(Debug, Deserialize, Serialize)]
pub struct WorkflowPrepareExecution {
    pub id: String,
    pub nodes: Value,
}

impl WorkflowPrepareExecution {
    fn get_executor(&self) -> Option<WorkflowExecutor> {
        match from_value::<BTreeMap<String, Node>>(self.nodes.clone()) {
            Ok(nodes) => {
                Some(
                    WorkflowExecutor {
                        id: self.id.clone(),
                        nodes,
                    }
                )
            }
            Err(_) => None
        }
    }
}