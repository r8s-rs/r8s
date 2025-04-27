-- Add migration script here
CREATE TABLE "workflow"(
    "id" TEXT NOT NULL,
    "name" TEXT NOT NULL,
    "description" TEXT NOT NULL,
    "active" BOOLEAN NOT NULL DEFAULT true,
    "settings" jsonb NOT NULL,
    "nodes" jsonb NOT NULL,
    "created_at" TIMESTAMP(0) WITHOUT TIME ZONE NOT NULL,
    "updated_at" TIMESTAMP(0) WITHOUT TIME ZONE NOT NULL
);
ALTER TABLE
    "workflow" ADD PRIMARY KEY("id");
CREATE TABLE "execution"(
    "id" BIGINT NOT NULL,
    "previous_execution_id" BIGINT NULL,
    "workflow_id" TEXT NOT NULL,
    "status" VARCHAR(255) CHECK
        (
            "status" IN(
                'queued',
                'running',
                'canceled',
                'success',
                'error',
                'waiting'
            )
        ) NOT NULL,
        "node_key" TEXT NOT NULL
);
ALTER TABLE
    "execution" ADD PRIMARY KEY("id");
CREATE INDEX "execution_status_index" ON
    "execution"("status");
CREATE TABLE "execution_data"(
    "execution_id" BIGINT NOT NULL,
    "input" jsonb NULL,
    "output" jsonb NULL
);
ALTER TABLE
    "execution_data" ADD PRIMARY KEY("execution_id");
ALTER TABLE
    "execution" ADD CONSTRAINT "execution_workflow_id_foreign" FOREIGN KEY("workflow_id") REFERENCES "workflow"("id");
ALTER TABLE
    "execution" ADD CONSTRAINT "execution_id_foreign" FOREIGN KEY("id") REFERENCES "execution_data"("execution_id");
ALTER TABLE
    "execution" ADD CONSTRAINT "execution_previous_execution_id_foreign" FOREIGN KEY("previous_execution_id") REFERENCES "execution"("id");