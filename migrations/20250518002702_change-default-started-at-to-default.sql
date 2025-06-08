-- Add migration script here

ALTER TABLE execution ALTER COLUMN started_at DROP DEFAULT;
