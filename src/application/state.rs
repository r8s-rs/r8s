use crate::infrastructure::repositories::Workflow;
use crate::domain::entities::HttpRequest;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use sqlx::{Postgres, Pool};


pub struct State {
    pub db: Pool<Postgres>,

    // recebe todos os webhooks e e posteriormente envia pra workflow_pendings
    pub webhook_v1_pendings: Arc<Mutex<VecDeque<HttpRequest>>>,

    // Recebe os workflows que serão salvos no banco de dados
    pub workflow_pendings: Arc<Mutex<VecDeque<Workflow>>>,
}
