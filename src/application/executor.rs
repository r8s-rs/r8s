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
        let node = workflow.nodes.get(&current_id).expect("Não encontrado");
        execute_node(node, &mut context, &mut queue);
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
        NodeKind::SetV1(set_node) => {
            for (key, val) in &set_node.params {
                ctx.memory.insert(key.clone(), val.clone());
            }

            println!("   ➥ Set vars: {:?}", set_node.params);

            for next_node in node.next.iter() {
                queue.push_back(next_node.clone());
            }
        }
        NodeKind::IfV1(if_node) => {
            let cond = &if_node.condition;
            let var_name = cond.split('.').nth(1).unwrap_or("").split_whitespace().next().unwrap_or("");
            let comparison = cond.split('>').nth(1).unwrap_or("").trim().parse::<i64>().unwrap_or(0);
            let var_value = ctx.memory.get(var_name).and_then(|v| v.as_i64()).unwrap_or(0);

            let branch = if var_value > comparison { "true" } else { "false" };
            println!("   ➥ Condição '{}' avaliou para {}", cond, branch);

            /*
            if let Some(next_id) = node.next.get(branch) {
                queue.push_back(next_id.clone());
            }
            */
        }
        NodeKind::SwitchV1(switch_node) => {
            let key = switch_node.key.split('.').nth(1).unwrap_or("");
            let value = ctx.memory.get(key).and_then(|v| v.as_i64()).unwrap_or(0);
        
            let next = if value < 18 {
                "0-17"
            } else if value < 65 {
                "18-64"
            } else {
                "65+"
            };
        
            println!("   ➥ Switch: valor '{}' mapeado para '{}'", value, next);
            if let Some(next_id) = switch_node.cases.get(next) {

                queue.push_back(next_id.clone());
            }
        }
        
        NodeKind::Unknown => println!("⚠️  Tipo desconhecido"),
    }
}