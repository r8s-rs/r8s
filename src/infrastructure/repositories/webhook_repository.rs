use sqlx::{PgPool, Error};
use super::Webhook;

pub struct WebhookRepository;

impl WebhookRepository {
    pub async fn get_by_path(pool: &PgPool, path: &str, method: &String) -> Result<Option<Webhook>, Error> {
        sqlx::query_as!(
            Webhook,
            r#"
            select
                response_code,
                workflow_id
            from
                webhook
            where
                path = $1
                and method = $2
            "#,
            path,
            method,
        ).fetch_optional(
            pool,
        ).await
    }
}