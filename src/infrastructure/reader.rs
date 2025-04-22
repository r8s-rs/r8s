use crate::domain::entities::Workflow;
use std::fs;

pub fn read_workflow(path: &str) -> Workflow {
    let json = fs::read_to_string(path).expect("Erro ao ler JSON");
    serde_json::from_str(&json).expect("Erro ao fazer parse do JSON")
}
