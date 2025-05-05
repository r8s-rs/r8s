use sqlx::{Transaction, Error, Postgres};
use std::collections::BTreeMap;
//use serde_json::json;
use super::Workflow;

type MapNodes<'a> = BTreeMap<&'a str, i64>;
type MapEdges = BTreeMap<i64, i64>;

pub struct ExecutionRepository;

impl ExecutionRepository {
    pub async fn insert(tx: &mut Transaction<'_, Postgres>) -> Result<(), Error> {
        /*
        let exists = sqlx::query_scalar!(
            r#"
            select
	            exists(
                    select 1 from workflow where id = $1
                ) as exists
            "#,
            email,
        )
        .fetch_one(&mut **tx)
        .await;

        dbg!(exists);
        
        let wf_inserted = wf_inserted.unwrap().id;
        
        let mut map_nodes: MapNodes = BTreeMap::new();
        
        let mut map_edges: MapEdges = BTreeMap::new();
        
        Self::insert_nodes(tx, wf_inserted, wf, &mut map_nodes, &mut map_edges).await;
        
        Self::insert_edges(tx, wf, &mut map_nodes, &mut map_edges).await;
        */

        Ok(())
    }

    async fn insert_nodes<'a>(tx: &mut Transaction<'_, Postgres>, wf_id: i64, wf: &'a Workflow, map_nodes: &mut MapNodes<'a>, map_edges: &mut MapEdges) {
        /*
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
        */
    }
}