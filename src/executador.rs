use chrono::{Utc, Duration};
use serde_json::Value;
use sqlx::{PgPool, Row};

/// Após executar um nó, programe os próximos nós conforme edges e delays
pub async fn enqueue_next_nodes(
    pool: &PgPool,
    execution_id: i64,
    current_node_id: i64,
    parent_output: Option<Value>,
) -> Result<(), sqlx::Error> {
    // 1. Buscar edges de saída do nó atual
    let edges = sqlx::query!(
        r#"
        SELECT to_node_id, condition
        FROM edge
        WHERE from_node_id = $1
        "#,
        current_node_id
    )
    .fetch_all(pool)
    .await?;

    for edge in edges {
        // 2. Verificar a condição (simplificado - se quiser posso incluir lógica de avaliação)
        if let Some(condition_json) = &edge.condition {
            if !evaluate_condition(condition_json, &parent_output) {
                continue;
            }
        }

        // 3. Buscar informações do nó destino
        let node = sqlx::query!(
            r#"
            SELECT id, data
            FROM node
            WHERE id = $1
            "#,
            edge.to_node_id
        )
        .fetch_one(pool)
        .await?;

        // 4. Checar se há um delay no campo data
        let delay_seconds = node
            .data
            .as_ref()
            .and_then(|d| d.get("delay"))
            .and_then(|d| d.as_i64())
            .unwrap_or(0);

        let scheduled_for = Utc::now() + Duration::seconds(delay_seconds);

        // 5. Inserir no execution_log para posterior execução
        sqlx::query!(
            r#"
            INSERT INTO execution_log (execution_id, node_id, scheduled_for)
            VALUES ($1, $2, $3)
            "#,
            execution_id,
            node.id,
            scheduled_for
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}

/// Lógica dummy para avaliação de condição (apenas retorna true por enquanto)
pub fn evaluate_condition(_cond: &Value, _input: &Option<Value>) -> bool {
    // Implementação real deveria interpretar o JSON
    true
}
