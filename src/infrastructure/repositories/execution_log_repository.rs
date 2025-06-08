use sqlx::{Transaction, Error, Postgres};
use serde_json::Value;

pub struct ExecutionLogRepository;

impl ExecutionLogRepository {

    pub async fn insert(tx: &mut Transaction<'_, Postgres>, execution_id: i64, node_id: i64, output: Option<Value>, error: Option<String>) -> Result<(), Error> {
        sqlx::query!(
            r#"
                insert into execution_log (
                    execution_id,
                    node_id,
                    finished_at,
                    output,
                    error
                ) values (
                    $1,
                    $2,
                    now(),
                    $3,
                    $4
                )
            "#,
            execution_id,
            node_id,
            output,
            error,
        ).execute(&mut **tx).await?;

        Ok(())
    }
}