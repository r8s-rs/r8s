use crate::infrastructure::repositories::Workflow;
use super::{Partitions, TemplateRender};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use sqlx::{Postgres, Pool};
use fjall::Keyspace;


pub struct State {
    pub db: Pool<Postgres>,

    pub keyspace: Arc<Keyspace>,
    pub partitions: Arc<Partitions>,

    // Recebe os workflows que serão salvos no banco de dados
    pub workflow_pendings: Arc<Mutex<VecDeque<Workflow>>>,
    pub template_render: TemplateRender,
}
