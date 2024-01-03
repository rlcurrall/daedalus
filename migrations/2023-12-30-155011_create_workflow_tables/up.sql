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
    id UUID PRIMARY KEY,
    tenant_id INT NOT NULL REFERENCES tenants(id),
    name VARCHAR(255) NOT NULL,
    description TEXT,
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

-- Create workflow state table
CREATE TABLE workflow_states (
    id UUID PRIMARY KEY,
    workflow_id UUID NOT NULL REFERENCES workflows(id),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE
);

-- Add `initial_state_id` column to workflow table
ALTER TABLE workflows
  ADD COLUMN initial_state_id UUID REFERENCES workflow_states(id);

-- Track updated_at column
CREATE TRIGGER update_workflow_states_updated_at
  BEFORE UPDATE
  ON
    workflow_states
  FOR EACH ROW
EXECUTE PROCEDURE track_updated_at();

-- Create workflow actions table
CREATE TABLE workflow_actions (
    id UUID PRIMARY KEY,
    state_id UUID NOT NULL REFERENCES workflow_states(id),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    timing VARCHAR(255) NOT NULL, -- entry or exit
    configuration JSONB,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE
);

-- Track updated_at column
CREATE TRIGGER update_workflow_actions_updated_at
  BEFORE UPDATE
  ON
    workflow_actions
  FOR EACH ROW
EXECUTE PROCEDURE track_updated_at();

-- Create workflow triggers table
CREATE TABLE workflow_triggers (
    id UUID PRIMARY KEY,
    state_id UUID NOT NULL REFERENCES workflow_states(id),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    configuration JSONB,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE
);

-- Track updated_at column
CREATE TRIGGER update_workflow_triggers_updated_at
  BEFORE UPDATE
  ON
    workflow_triggers
  FOR EACH ROW
EXECUTE PROCEDURE track_updated_at();

