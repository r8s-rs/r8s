use crate::infrastructure::repositories::{WebhookRepository, WorkflowRepository};
use actix::{Actor, Context, spawn};
use crate::application::State;
use log::{error, info, warn};
use actix_web::web::Data;
use actix::AsyncContext;
use std::time::Duration;

pub struct WebhookV1ToWorkflow {
    pub state: Data<State>,
}

impl Actor for WebhookV1ToWorkflow {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("[WebhookV1ToWorkflow] Iniciado.");

        let db = self.state.db.clone();
        let webhook_v1_pendings = self.state.webhook_v1_pendings.clone();

        ctx.run_interval(Duration::from_secs(30), move |_actor, _ctx| {
            let db = db.clone();
            let webhook_v1_pendings = webhook_v1_pendings.clone();

            spawn(async move {
                info!("[WebhookV1ToWorkflow] Executando tarefa async a cada 30 segundos...");

                let mut wh_pendings = webhook_v1_pendings.try_lock().unwrap();

                dbg!(&wh_pendings);

                let keys: Vec<_> = wh_pendings.keys().cloned().collect();

                for wf_id in keys {
                    if let Some(http_reqs) = wh_pendings.get(&wf_id) {
                        match db.begin().await {
                            Ok(mut tx) => {
                                let exists = WorkflowRepository::exists_by_id(&mut tx, wf_id).await;
                                match exists {
                                    Ok(true) => {
                                        let res = WebhookRepository::insert_execution(
                                            &mut tx,
                                            wf_id,
                                            http_reqs,
                                        ).await;
            
                                        match res {
                                            Ok(()) => {
                                                match tx.commit().await {
                                                    Ok(_) => {
                                                        wh_pendings.remove(&wf_id);
                                                        info!("Foram inseridas novas execuções para o Workflow {wf_id}");
                                                    }
                                                    Err(e) => {
                                                        error!("Erro ao commitar transação: {e}");
                                                    }
                                                }
                                            }
                                            Err(insert_e) => {
                                                match tx.rollback().await {
                                                    Ok(_) => {
                                                        error!("Erro ao inserir execuções para o Workflow {wf_id}, rollback feito: {insert_e}");
                                                    }
                                                    Err(e) => {
                                                        error!("Erro ao inserir execuções para o Workflow {wf_id}, falha ao fazer rollback: {e}");
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    Ok(false) => {
                                        warn!("workflow não encontrado");
                                    }
                                    Err(e) => {
                                        error!("Erro ao verificar existência do workflow {wf_id}: {e}");
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Erro ao iniciar transação: {e}");
                            }
                        }
                    }
                }
            });
        });
    }
}