use sqlx::{Transaction, Error, Postgres};
use std::collections::BTreeMap;
use serde_json::json;
use super::Workflow;

type MapNodes<'a> = BTreeMap<&'a str, i64>;
type MapEdges = BTreeMap<i64, i64>;

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

        let mut map_edges: MapEdges = BTreeMap::new();

        Self::insert_nodes(tx, wf_inserted, wf, &mut map_nodes, &mut map_edges).await;

        Self::insert_edges(tx, wf, &mut map_nodes, &mut map_edges).await;

        Ok(())
    }

    async fn insert_nodes<'a>(tx: &mut Transaction<'_, Postgres>, wf_id: i64, wf: &'a Workflow, map_nodes: &mut MapNodes<'a>, map_edges: &mut MapEdges) {
        for (node_key, node) in &wf.nodes {
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
                json!({}),
            ).fetch_one(
                &mut **tx,
            ).await;

            let node_inserted_id = node_inserted.unwrap().id;

            map_nodes.insert(
                node_key,
                node_inserted_id,
            );
        }
    }

    async fn insert_edges(tx: &mut Transaction<'_, Postgres>, wf: &Workflow, map_nodes: &mut BTreeMap<&str, i64>, map_edges: &mut BTreeMap<i64, i64>) {
        for (node_key, node) in &wf.nodes {
            let from_node_id = map_nodes[node_key.as_str()];

            if let Some(edges) = &node.next {
                for edge in edges {
                    dbg!(&edge);
                    let to_node_id = map_nodes.get(edge.as_str());

                    if to_node_id.is_none() {
                        continue;
                    }

                    let node_inserted = sqlx::query!(
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