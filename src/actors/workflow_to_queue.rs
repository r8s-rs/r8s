use crate::infrastructure::repositories::{ExecutionRepository, Workflow};
use crate::domain::entities::{ExecutionStatus, Node, Edge};
use actix::{Actor, Context, AsyncContext, spawn};
use tracing::{info, error, trace};
use crate::application::Executor;
use std::collections::BTreeMap;
use crate::application::State;
use actix_web::web::Data;
use std::time::Duration;

pub struct WorkflowToQueue {
    pub state: Data<State>,
}

impl Actor for WorkflowToQueue {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("[WorkflowToQueue] Iniciado.");

        let db = self.state.db.clone();

        ctx.run_interval(Duration::from_secs(10), move |_actor, ctx| {
            let db = db.clone();

            // aqui verificar uso de memoria ram e cpu antes de pegar novas tarefas, o usuário deverá específicar a lógica usando variaveis de ambiente

            spawn(async move {
                match db.begin().await {
                    Ok(mut tx) => {
                        let executions = ExecutionRepository::get_by_status_with_lock(
                            &mut tx,
                            &ExecutionStatus::Queued,
                            10,
                        ).await;

                        if let Err(ref e) = executions {
                            error!("[WorkflowToQueue] Erro ao obter execuções: {}", e);
                        }

                        let executions = executions.unwrap();

                        for execution in executions {
                            info!("[WorkflowToQueue] Executando execução: {}", execution.id);

                            let _ = ExecutionRepository::update_status_to_running(
                                &mut tx,
                                execution.id
                            ).await.unwrap();

                            let edges: BTreeMap<i64, Edge> = ExecutionRepository::get_edges_by_workflow_id(&mut tx, execution.workflow_id, execution.id)
                                .await
                                .expect("get_edges_by_workflow_id failed")
                                .into_iter()
                                .map(|edge| (edge.from_id, edge))
                                .collect();

                            let mut last_node = None::<Node>;

                            let mut nodes = BTreeMap::new();

                            for edge in edges.values() {
                                let node = edge.get_from_node(&mut tx, last_node.as_ref()).await;

                                trace!("[WorkflowToQueue] Node: {:#?}, Edge: {:#?}", &edge, &node);

                                nodes.insert(
                                    edge.from_id as u64,
                                    node.clone()
                                );

                                last_node = Some(node)
                            }

                            let wf = Workflow {
                                id: Some(execution.workflow_id),
                                pub_id: "".to_string(),
                                name: None,
                                description: None,
                                nodes,
                            };

                            let mut executor = Executor::new(wf, execution.input, execution.id);

                            match executor.run(&mut tx, &edges).await {
                                Ok(ExecutionStatus::Success) => {
                                    info!("[WorkflowToQueue] Execução finalizada com sucesso: {}", execution.id);

                                    let _ = ExecutionRepository::update_status_to_finished(
                                        &mut tx,
                                        execution.id,
                                        &ExecutionStatus::Success
                                    ).await.unwrap();
                                }
                                Ok(ExecutionStatus::Error) => {
                                    info!("[WorkflowToQueue] Execução finalizada com erro: {}", execution.id);

                                    let _ = ExecutionRepository::update_status_to_finished(
                                        &mut tx,
                                        execution.id,
                                        &ExecutionStatus::Error
                                    ).await.unwrap();
                                }
                                Err(e) => {
                                    error!("[WorkflowToQueue] Erro ao executar execução: {}", e);
                                },
                                _ => {}
                            }
                        }

                        match tx.commit().await {
                            Ok(_) => {}
                            Err(e) => {
                                error!("[WorkflowToQueue] Erro ao commitar transação: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        error!("[WorkflowToQueue] Erro ao abrir transação: {}", e);
                    }
                }
            });
        });
    }
}