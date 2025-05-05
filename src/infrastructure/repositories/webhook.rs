use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Webhook {
    pub response_code: i16,
    pub workflow_id: i64,
}
