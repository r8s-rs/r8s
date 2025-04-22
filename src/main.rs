mod domain;

mod application {
    pub mod executor;
}
mod infrastructure {
    pub mod reader;
}

use application::executor::run_workflow;
use infrastructure::reader::read_workflow;

fn main() {
    let workflow = read_workflow("workflow.json");
    println!("🚀 Executando workflow '{}'", workflow.workflow_id);
    run_workflow(&workflow);
}