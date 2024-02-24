/*
|-------------------------------------------------------------------------------
| Create Workflow Tables
|-------------------------------------------------------------------------------
|
| This migration creates the workflow tables.
|
| @date 2023-12-30
| @author Robb Currall <robb@currall.net>
|
*/

-- Create workflow table
CREATE TABLE workflows (
    id BIGSERIAL PRIMARY KEY,
    tenant_id INT NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    definition JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE
);

-- Track updated_at column
CREATE TRIGGER update_workflow_updated_at
  BEFORE UPDATE
  ON
    workflows
  FOR EACH ROW
EXECUTE PROCEDURE track_updated_at();
