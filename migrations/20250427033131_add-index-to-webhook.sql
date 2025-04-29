-- Add migration script here
CREATE INDEX idx_webhook_path_method ON webhook (path, method);
