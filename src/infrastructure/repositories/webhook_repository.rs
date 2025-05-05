
use sqlx::{PgPool, Error, Transaction, Postgres};
use crate::domain::entities::HttpRequest;
use super::{Webhook, WebhookMethod};
use serde_json::json;

pub struct WebhookRepository;

impl WebhookRepository {
    pub async fn get_by_path(pool: &PgPool, path: &str, method: &WebhookMethod) -> Result<Option<Webhook>, Error> {
        sqlx::query_as(
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
        )
        .bind(path)
        .bind(method)
        .fetch_optional(
            pool,
        ).await
    }

    pub async fn insert_executions(
        tx: &mut Transaction<'_, Postgres>,
        wf_id: i64,
        objs: &Vec<HttpRequest>,
    ) -> Result<(), Error> {
        if objs.is_empty() {
            return Ok(());
        }

        // Serializa em um vetor de JSONs
        let inputs: Vec<serde_json::Value> = objs.iter().map(|item| json!(item)).collect();

        // Executa em lote
        sqlx::query!(
            r#"
            insert into execution (workflow_id, input)
            select
                $1 as workflow_id,
                unnest($2::jsonb[]) as input
            "#,
            wf_id,
            &inputs as &[serde_json::Value],
        )
        .execute(&mut **tx)
        .await?;

        Ok(())
    }
}