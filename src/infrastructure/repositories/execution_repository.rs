use crate::domain::entities::{Execution, ExecutionStatus, Edge};
use sqlx::{Transaction, Error, Postgres};
use tracing::{debug, info, trace};

pub struct ExecutionRepository;

impl ExecutionRepository {
    pub async fn get_by_status_with_lock(tx: &mut Transaction<'_, Postgres>, status: &ExecutionStatus, limit: i32) -> Result<Vec<Execution>, Error> {
        let executions: Vec<Execution> = sqlx::query_as(
            r#"
            select
                id,
                workflow_id,
                input
            from
                execution
            where
                status = $1
                and scheduled_for < now()
            for update
                skip locked
            limit $2
            "#,
        )
        .bind(status)
        .bind(limit)
        .fetch_all(&mut **tx)
        .await?;

        Ok(executions)
    }

    pub async fn update_status(tx: &mut Transaction<'_, Postgres>, id: i64, status: &ExecutionStatus) -> Result<(), Error> {
        sqlx::query(
            "update execution set status = $1 where id = $2"
        ).bind(
            status,
        ).bind(
            id
        ).execute(&mut **tx).await?;
        Ok(())
    }

    pub async fn update_status_to_running(tx: &mut Transaction<'_, Postgres>, id: i64) -> Result<(), Error> {
        sqlx::query(
            "update execution set status = $1, started_at = now() where id = $2"
        ).bind(
            ExecutionStatus::Running
        ).bind(
            id
        ).execute(&mut **tx).await?;

        Ok(())
    }

    pub async fn update_status_to_finished(tx: &mut Transaction<'_, Postgres>, id: i64, status: &ExecutionStatus) -> Result<(), Error> {
        sqlx::query(
            "update execution set status = $1, finished_at = now() where id = $2"
        ).bind(
            status,
        ).bind(
            id
        ).execute(&mut **tx).await?;

        Ok(())
    }

    pub async fn get_edges_by_workflow_id(tx: &mut Transaction<'_, Postgres>, workflow_id: i64, execution_id: i64) -> Result<Vec<Edge>, Error> {
        sqlx::query_as!(
            Edge,
            r#"
                select
                    n.id as from_id,
                    e.to_node_id to_id,
                    n.name as from_name,
                    n.data as from_data,
                    n.type as from_type,
                    el.output as from_output,
                    el.error as from_error,
                    e.condition,
                    n.workflow_id,
                    el.id as "execution_log_id?: _"
                from
                    node n
                join edge e on
                    n.id = e.from_node_id
                join node n2 on
                    e.to_node_id = n2.id
                join execution ex on
                    n.workflow_id = ex.workflow_id
                left join execution_log el on
                    ex.id = el.execution_id
                    and el.node_id = n.id
                where
                    n.workflow_id = $1
                    and ex.id = $2
                order by
                    n.key asc
            "#,
            workflow_id,
            execution_id,
        ).fetch_all(&mut **tx).await
    }
}