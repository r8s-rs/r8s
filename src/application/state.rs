use crate::infrastructure::repositories::Workflow;
use std::collections::{VecDeque, BTreeMap};
use crate::domain::entities::HttpRequest;
use std::sync::{Arc, Mutex};
use sqlx::{Postgres, Pool};
use super::Partitions;
use fjall::Keyspace;


pub struct State {
    pub db: Pool<Postgres>,

    pub keyspace: Arc<Keyspace>,
    pub partitions: Arc<Partitions>,

    // Recebe os workflows que serão salvos no banco de dados
    pub workflow_pendings: Arc<Mutex<VecDeque<Workflow>>>,
}
