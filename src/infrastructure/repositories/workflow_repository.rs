use crate::domain::workflow::WebhookV1Node;
use sqlx::{Transaction, Error, Postgres};
use crate::domain::entities::NodeKind;
use std::collections::BTreeMap;
use tracing::{info, trace};
use serde_json::json;
use super::Workflow;

type MapNodes<'a> = BTreeMap<u64, i64>;

pub struct WorkflowRepository;

impl WorkflowRepository {
    pub async fn exists_by_id(tx: &mut Transaction<'_, Postgres>, id: i64) -> Result<bool, Error> {
        let exists = sqlx::query_scalar!(
            r#"
            select
                exists(
                    select 1 from workflow where id = $1
                ) as exists
            "#,
            id,
        )
        .fetch_one(&mut **tx)
        .await?;

        Ok(exists.unwrap())
    }

    pub async fn insert(tx: &mut Transaction<'_, Postgres>, wf: &Workflow) -> Result<(), Error> {
        sqlx::query!(
            "update workflow set pub_id = null where pub_id = $1",
            wf.pub_id
        ).execute(&mut **tx).await?;

        let wf_inserted = sqlx::query!(
            r#"
            insert into workflow (
                pub_id,
                name,
                description
            ) values (
                $1,
                $2,
                $3
            ) returning id"#,
            wf.pub_id,
            wf.name,
            wf.description,
        ).fetch_one(
            &mut **tx,
        ).await;

        let wf_inserted = wf_inserted.unwrap().id;

        let mut map_nodes: MapNodes = BTreeMap::new();

        Self::prepare_triggers(tx, wf_inserted, wf).await;

        Self::insert_nodes(tx, wf_inserted, wf, &mut map_nodes).await;

        Self::insert_edges(tx, wf, &mut map_nodes).await;

        Ok(())
    }

    async fn prepare_triggers<'a>(tx: &mut Transaction<'_, Postgres>, wf_id: i64, wf: &'a Workflow) {
        for (_, node) in &wf.nodes {
            if !node.get_kind().is_trigger() {
                continue;
            }

            match node.kind {
                NodeKind::WebhookV1(ref kind) => Self::insert_webhook_v1(tx, kind, wf_id).await,
                _ => ()
            }

            break;
        }
    }

    async fn insert_webhook_v1<'a>(tx: &mut Transaction<'_, Postgres>, node: &WebhookV1Node, wf_id: i64) {
        let _ = sqlx::query(
                r#"
                INSERT INTO webhook (
                    path,
                    method,
                    workflow_id,
                    response_code
                ) VALUES (
                    $1,
                    $2,
                    $3,
                    $4
                ) ON CONFLICT (path)
                DO UPDATE SET
                    method = EXCLUDED.method,
                    workflow_id = EXCLUDED.workflow_id,
                    response_code = EXCLUDED.response_code
                RETURNING path
                "#,
            )
            .bind(&node.path)
            .bind(&node.method)
            .bind(wf_id)
            .bind(node.response_code)
            .fetch_one(&mut **tx)
            .await;

        trace!(
            path = &node.path,
            method = &node.method.to_string(),
            wf_id = wf_id,
            response_code = node.response_code,
        );
    }

    async fn insert_nodes<'a>(tx: &mut Transaction<'_, Postgres>, wf_id: i64, wf: &'a Workflow, map_nodes: &mut MapNodes<'a>) {
        for (node_key, node) in &wf.nodes {
            trace!(node_key, node.name);

            let node_kind = node.get_kind();

            let node_type = node_kind.get_type();

            let node_inserted = sqlx::query!(
                r#"
                insert into node (
                    workflow_id,
                    name,
                    type,
                    data
                ) values (
                    $1,
                    $2,
                    $3,
                    $4
                ) returning id"#,
                wf_id,
                node.name,
                node_type,
                json!(node.conditions),
            ).fetch_one(
                &mut **tx,
            ).await;

            let node_inserted_id = node_inserted.unwrap().id;

            map_nodes.insert(
                *node_key,
                node_inserted_id,
            );
        }
    }

    async fn insert_edges(tx: &mut Transaction<'_, Postgres>, wf: &Workflow, map_nodes: &mut MapNodes<'_>) {
        for (node_key, node) in &wf.nodes {
            let from_node_id = map_nodes[node_key];

            info!(node_key, from_node_id);

            if let Some(edges) = &node.next {
                for edge in edges {
                    let to_node_id = map_nodes.get(edge);

                    if to_node_id.is_none() {
                        trace!(reason = "to_node_id not_found", node = edge);
                        continue;
                    }

                    let _ = sqlx::query!(
                        r#"
                        insert into edge (
                            from_node_id,
                            to_node_id,
                            condition
                        ) values (
                            $1,
                            $2,
                            $3
                        )"#,
                        from_node_id,
                        to_node_id,
                        json!(node.conditions),
                    ).fetch_one(
                        &mut **tx,
                    ).await;
                }
            }
        }
    }
}