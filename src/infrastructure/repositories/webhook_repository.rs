
use sqlx::{PgPool, Error, Transaction, Postgres};
use crate::domain::entities::HttpRequest;
use std::collections::VecDeque;
use serde_json::json;
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

    pub async fn insert_execution(tx: &mut Transaction<'_, Postgres>, wf_id: i64, objs: &VecDeque<HttpRequest>) -> Result<(), Error> {
        for item in objs {
            sqlx::query!(
                r#"
                insert into execution (
                    workflow_id,
                    input
                ) values (
                    $1,
                    $2
                ) returning id
                "#,
                wf_id,
                json!(item),
            )
            .fetch_one(&mut **tx)
            .await?;
        }

        Ok(())
    }
}