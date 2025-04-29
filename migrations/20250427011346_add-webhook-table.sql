-- Add migration script here
CREATE TABLE "webhook"(
    "path" TEXT NOT NULL,
    "method" VARCHAR(255) CHECK
        (
            "method" IN(
                'get',
                'post',
                'delete',
                'put',
                'patch',
                'head'
            )
        ) NOT NULL,
    "workflow_id" TEXT NOT NULL,
    "response_code" SMALLINT NOT NULL DEFAULT 200
);
ALTER TABLE
    "webhook" ADD PRIMARY KEY("path");
ALTER TABLE
    "webhook" ADD CONSTRAINT "webhook_workflow_id_foreign" FOREIGN KEY("workflow_id") REFERENCES "workflow"("id");