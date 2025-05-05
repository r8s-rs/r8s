-- Add migration script here
ALTER TABLE public.execution_log
ADD COLUMN scheduled_for timestamptz DEFAULT now();
