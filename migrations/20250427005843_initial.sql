CREATE TABLE workflow (
    id BIGSERIAL PRIMARY KEY,
    pub_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ DEFAULT now(),
    updated_at TIMESTAMPTZ DEFAULT now()
);

CREATE TABLE node (
    id BIGSERIAL PRIMARY KEY,
    workflow_id BIGINT NOT NULL REFERENCES workflow(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    type TEXT NOT NULL,
    data JSONB,
    position_x NUMERIC NOT NULL DEFAULT 0,
    position_y NUMERIC NOT NULL DEFAULT 0
);

CREATE TABLE edge (
    from_node_id BIGINT NOT NULL REFERENCES node(id) ON DELETE CASCADE,
    to_node_id BIGINT NOT NULL REFERENCES node(id) ON DELETE CASCADE,
    PRIMARY KEY (from_node_id, to_node_id)
);

CREATE TYPE execution_status AS ENUM (
    'queued',
    'running',
    'canceled',
    'success',
    'error',
    'waiting'
);

CREATE TABLE execution (
    id BIGSERIAL PRIMARY KEY,
    workflow_id BIGINT NOT NULL REFERENCES workflow(id) ON DELETE CASCADE,
    status execution_status NOT NULL DEFAULT 'queued',
    scheduled_for TIMESTAMPTZ NOT NULL DEFAULT now(),
    started_at TIMESTAMPTZ DEFAULT now(),
    finished_at TIMESTAMPTZ,
    input JSONB
);

CREATE TABLE execution_log (
    id BIGSERIAL PRIMARY KEY,
    execution_id BIGINT NOT NULL REFERENCES execution(id) ON DELETE CASCADE,
    node_id BIGINT NOT NULL REFERENCES node(id) ON DELETE CASCADE,
    started_at TIMESTAMPTZ DEFAULT now(),
    finished_at TIMESTAMPTZ,
    output JSONB,
    error TEXT
);

-- Índices para performance
CREATE UNIQUE INDEX idx_workflow_pub_id ON workflow(pub_id);
CREATE INDEX idx_node_workflow ON node(workflow_id);
CREATE INDEX idx_edge_from_node ON edge(from_node_id);
CREATE INDEX idx_edge_to_node ON edge(to_node_id);
CREATE INDEX idx_execution_workflow ON execution(workflow_id);
CREATE INDEX idx_execution_status ON execution(status);
CREATE INDEX idx_execution_log_exec ON execution_log(execution_id);
