-- Add migration script here
ALTER TABLE edge
ADD COLUMN condition JSONB;

CREATE INDEX idx_edge_condition ON edge
USING GIN (condition jsonb_path_ops);

COMMENT ON COLUMN edge.condition IS
'Condição JSONB avaliada no momento da execução. Pode conter "and", "or", "left", "op", "right".';
