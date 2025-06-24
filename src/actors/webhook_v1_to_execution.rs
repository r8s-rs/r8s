use crate::infrastructure::repositories::{WebhookRepository, WorkflowRepository};
use std::{collections::BTreeMap, time::Duration};
use crate::domain::entities::HttpRequest;
use actix::{Actor, Context, spawn};
use tracing::{error, info, warn};
use crate::application::State;
use actix_web::web::Data;
use actix::AsyncContext;

pub struct WebhookV1ToExecution {
    pub state: Data<State>,
}

impl Actor for WebhookV1ToExecution {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("[WebhookV1ToExecution] Iniciado.");

        let db = self.state.db.clone();

        let partitions = self.state.partitions.clone();

        ctx.run_interval(Duration::from_secs(30), move |_actor, _ctx| {
            let db = db.clone();
            let webhook_v1_pendings = partitions.webhook_v1_pendings.clone();

            let partitions = partitions.clone();

            spawn(async move {
                info!("[WebhookV1ToExecution] Executando tarefa async a cada 30 segundos...");

                let mut keys = BTreeMap::new();

                for key in webhook_v1_pendings.keys() {
                    if let Ok(key) = key {
                        let key = String::from_utf8(key.to_vec()).unwrap();

                        let wf_id = key.clone().leak().split('/').next().unwrap();

                        keys.entry(wf_id).or_insert(vec![]);

                        if let Some(val) = keys.get_mut(&wf_id) {
                            val.push(key.leak());
                        };
                    }
                }

                for (wf_id, http_reqs_ids) in keys {
                    match db.begin().await {
                        Ok(mut tx) => {
                            let wf_id_int = wf_id.parse::<i64>().unwrap();

                            let exists = WorkflowRepository::exists_by_id(&mut tx, wf_id_int).await;

                            let mut map_insert = BTreeMap::new();

                            match exists {
                                Ok(true) => {
                                    for http_req_id in http_reqs_ids {
                                        let http_req_id = &*http_req_id;

                                        match partitions.webhook_v1_pendings.get(http_req_id) {
                                            Ok(wh_pending) => match wh_pending {
                                                Some(wh_pending) => {
                                                    let wh_pending: HttpRequest = serde_json::from_slice(&wh_pending).unwrap();
                                                    let req_id = http_req_id.to_owned();

                                                    map_insert.insert(req_id, wh_pending);
                                                },
                                                None => {
                                                    info!("Não encontrado wh na partition com id {http_req_id}");
                                                }
                                            }
                                            Err(e) => {
                                                error!("Erro ao obter wh da partition: {e}");
                                            }
                                        };
                                    }
                                }
                                Ok(false) => {
                                    let _ = webhook_v1_pendings.remove(wf_id);
                                    warn!("workflow não encontrado");
                                }
                                Err(e) => {
                                    error!("Erro ao verificar existência do workflow {wf_id}: {e}");
                                }
                            };

                            let inserts: Vec<HttpRequest> = map_insert.values().cloned().collect();

                            let res = WebhookRepository::insert_executions(
                                &mut tx,
                                wf_id_int,
                                &inserts,
                            ).await;

                            match res {
                                Ok(()) => {
                                    match tx.commit().await {
                                        Ok(_) => {
                                            info!("Foram inseridas [{}] novas execuções para o Workflow [{wf_id}]", inserts.len());

                                            for key in map_insert.keys() {
                                                match partitions.webhook_v1_pendings.remove(key) {
                                                    Ok(_) => info!("WH removido da fila {key}"),
                                                    Err(e) => error!("Falha ao remover wh [{key}] da fila: {e}"),
                                                };
                                            }
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
                        Err(e) => {
                            error!("Erro ao iniciar transação: {e}");
                        }
                    }
                }
            });
        });
    }
}