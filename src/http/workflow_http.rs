use crate::infrastructure::repositories::{WorkflowRepository, Workflow as WorkflowEntity};
use tracing::info;
use crate::State;
use actix_web::{
    HttpResponse,
    Responder,
    web::{
        Data,
        Json
    },
};

pub struct WorkflowHttp;

impl WorkflowHttp {
    pub async fn store(wf: Json<WorkflowEntity>, state: Data<State>) -> impl Responder {
        let wf = wf.into_inner();

        info!("Saving workflow [{}]", wf.pub_id);

        match state.db.begin().await {
            Ok(mut tx) => {
                match WorkflowRepository::insert(&mut tx, &wf).await {
                    Ok(_) => {
                        let _ = tx.commit().await;

                        HttpResponse::Ok().finish()
                    }
                    Err(e) => {
                        let _ = tx.rollback().await;

                        HttpResponse::InternalServerError().body(e.to_string())
                    }
                }
            },
            Err(e) => HttpResponse::InternalServerError().body(e.to_string())
        }
    }
}