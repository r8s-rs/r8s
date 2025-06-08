-- Add migration script here

CREATE UNIQUE INDEX idx_execution_log_execution_id_node_id ON execution_log (execution_id, node_id);