use crate::infrastructure::repositories::{WorkflowRepository, Workflow as WorkflowEntity};
use crate::State;
use log::info;
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

        let _ = WorkflowRepository::insert(&state.db, &wf).await;

        HttpResponse::Ok().finish()
    }
}