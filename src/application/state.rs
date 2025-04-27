use sqlx::{Postgres, Pool};

pub struct State {
    pub db: Pool<Postgres>,
}
