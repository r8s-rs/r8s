use actix::{Actor, Context, AsyncContext};
use crate::application::State;
use actix_web::web::Data;
use std::time::Duration;

pub struct WorkflowToQueue {
    pub state: Data<State>,
}

impl Actor for WorkflowToQueue {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("[WorkflowToQueue] Iniciado.");

        let mut workflow_pendings = self.state.workflow_pendings.clone();

        ctx.run_interval(Duration::from_secs(10), move |_actor, _ctx| {
            println!("[WorkflowToQueue] Executando tarefa a cada 30 segundos...");
            // Sua lógica de tarefa recorrente aqui

            dbg!(&workflow_pendings);
        });
    }
}