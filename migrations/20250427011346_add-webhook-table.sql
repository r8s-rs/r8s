-- Add migration script here
CREATE TYPE webhook_method AS ENUM (
    'get',
    'post',
    'delete',
    'put',
    'patch',
    'head'
);

CREATE TABLE webhook (
    path TEXT NOT NULL PRIMARY KEY,
    method webhook_method NOT NULL,
    workflow_id BIGINT NOT NULL REFERENCES workflow(id) ON DELETE CASCADE,
    response_code SMALLINT NOT NULL DEFAULT 200
);

CREATE INDEX idx_webhook_path_method ON webhook (path, method);