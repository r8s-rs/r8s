use actix::{Actor, Context, AsyncContext};
use crate::application::State;
use actix_web::web::Data;
use std::time::Duration;

pub struct WebhookV1ToWorkflow {
    pub state: Data<State>,
}

impl Actor for WebhookV1ToWorkflow {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("[WebhookV1ToWorkflow] Iniciado.");

        let mut webhook_v1_pendings = self.state.webhook_v1_pendings.clone();

        ctx.run_interval(Duration::from_secs(10), move |_actor, _ctx| {
            
            println!("[WebhookV1ToWorkflow] Executando tarefa a cada 30 segundos...");
            // Sua lógica de tarefa recorrente aqui

            dbg!(&webhook_v1_pendings);
        });
    }
}