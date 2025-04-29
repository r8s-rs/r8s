use std::collections::{HashMap, VecDeque};
use crate::domain::entities::*;

#[derive(Default)]
pub struct ExecutionContext {
    pub memory: HashMap<String, serde_json::Value>,
}

pub fn run_workflow(workflow: &Workflow) {
    let mut queue = VecDeque::new();
    queue.push_back(workflow.start.clone());
    let mut context = ExecutionContext::default();

    while let Some(current_id) = queue.pop_front() {

        if let Some(node) = workflow.nodes.get(&current_id) {
            execute_node(node, &mut context, &mut queue);
        }
    }

    println!("✅ Fluxo finalizado. Contexto: {:?}", context.memory);
}

fn execute_node(node: &Node, ctx: &mut ExecutionContext, queue: &mut VecDeque<String>) {
    println!("🔹 Executando '{}'", node.name);

    match &node.kind {
        NodeKind::ManualTriggerV1(_) => {
            println!("   ➥ Manual Trigger");

            for next_node in node.next.iter() {
                queue.push_back(next_node.clone());
            }
        }
        NodeKind::WebhookV1(webhook_node) => {
            //
        }
        NodeKind::SetV1(set_node) => {
            println!("   ➥ Set vars: {:?}", set_node.data);

            for next_node in node.next.iter() {
                queue.push_back(next_node.clone());
            }
        }
        NodeKind::Unknown => println!("⚠️  Tipo desconhecido"),
    }
}