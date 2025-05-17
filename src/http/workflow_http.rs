use crate::infrastructure::repositories::{WorkflowRepository, Workflow as WorkflowEntity};
use tracing::{info, trace};
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

        let (mut has_trigger, mut has_node) = (false, false);

        for node in wf.nodes.values() {
            if has_trigger && has_node {
                break;
            }

            let kind = node.get_kind();

            if !has_trigger && kind.is_trigger() {
                has_trigger = node
                    .next
                    .as_ref()
                    .map_or(false, |next| next.iter().any(|key| wf.nodes.contains_key(key)));
            }

            if !has_node && !kind.is_trigger() {
                has_node = true;
            }
        }

        trace!(has_trigger, has_node);

        if !has_trigger || !has_node {
            return HttpResponse::BadRequest()
                .body("The workflow must have at least one trigger and one node connected");
        }

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