/*
|-------------------------------------------------------------------------------
| Create users table
|-------------------------------------------------------------------------------
|
| This migration creates the users table.
|
| @date 2023-12-28
| @author Robb Currall <robb@currall.net>
|
*/

-- Create users table
CREATE TABLE IF NOT EXISTS users (
  id BIGSERIAL PRIMARY KEY,
  tenant_id INTEGER NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
  email VARCHAR(255) NOT NULL,
  password VARCHAR(255) NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  deleted_at TIMESTAMP WITH TIME ZONE
);

-- Create a unique index on the email column
CREATE UNIQUE INDEX users_email_unique ON users (tenant_id, email);

-- Create a trigger to update the updated_at column on every update
CREATE TRIGGER update_users_updated_at
  BEFORE UPDATE
  ON
    users
  FOR EACH ROW
EXECUTE PROCEDURE track_updated_at();
