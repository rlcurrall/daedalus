/*
|-------------------------------------------------------------------------------
| Drop Workflow Tables
|-------------------------------------------------------------------------------
|
| This migration drops the workflow tables.
|
| @date 2023-12-30
| @author Robb Currall <robb@currall.net>
|
*/

-- Drop workflow triggers table
DROP TRIGGER IF EXISTS update_workflow_triggers_updated_at ON workflow_triggers;
DROP TABLE IF EXISTS workflow_triggers;

-- Drop workflow actions table
DROP TRIGGER IF EXISTS update_workflow_actions_updated_at ON workflow_actions;
DROP TABLE IF EXISTS workflow_actions;

-- Remove `initial_state_id` column from workflow table
ALTER TABLE workflows
  DROP COLUMN initial_state_id;

-- Drop workflow state table
DROP TRIGGER IF EXISTS update_workflow_states_updated_at ON workflow_states;
DROP TABLE IF EXISTS workflow_states;

-- Drop workflow table
DROP TRIGGER IF EXISTS update_workflow_updated_at ON workflows;
DROP TABLE IF EXISTS workflows;
