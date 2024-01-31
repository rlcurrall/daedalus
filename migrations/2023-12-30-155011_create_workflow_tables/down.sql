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

-- Drop workflow table
DROP TRIGGER IF EXISTS update_workflow_updated_at ON workflows;
DROP TABLE IF EXISTS workflows;
