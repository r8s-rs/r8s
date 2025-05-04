use crate::infrastructure::repositories::Workflow;
use std::collections::{VecDeque, BTreeMap};
use crate::domain::entities::HttpRequest;
use std::sync::{Arc, Mutex};
use sqlx::{Postgres, Pool};


pub struct State {
    pub db: Pool<Postgres>,

    // recebe todos os webhooks e e posteriormente envia pra workflow_pendings
    pub webhook_v1_pendings: Arc<Mutex<BTreeMap<i64, VecDeque<HttpRequest>>>>,

    // Recebe os workflows que serão salvos no banco de dados
    pub workflow_pendings: Arc<Mutex<VecDeque<Workflow>>>,
}
