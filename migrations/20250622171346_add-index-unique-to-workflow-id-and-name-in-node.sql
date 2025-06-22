-- Add migration script here

CREATE UNIQUE INDEX IF NOT EXISTS idx_workflow_id_name_unique ON node (workflow_id, name);