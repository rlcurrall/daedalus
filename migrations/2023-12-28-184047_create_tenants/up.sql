/*
|-------------------------------------------------------------------------------
| Create tenants table
|-------------------------------------------------------------------------------
|
| This migration creates the tenants table.
|
| @date 2023-12-28
| @author Robb Currall <robb@currall.net>
|
*/

-- Create a function to update the updated_at column on every update
CREATE FUNCTION track_updated_at()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = now();
  RETURN NEW;
END;
$$ language 'plpgsql';

-- Create the tenants table
CREATE TABLE IF NOT EXISTS tenants (
  id SERIAL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  deleted_at TIMESTAMP WITH TIME ZONE
);

-- Create a trigger to update the updated_at column on every update
CREATE TRIGGER update_tenants_updated_at
  BEFORE UPDATE
  ON
    tenants
  FOR EACH ROW
EXECUTE PROCEDURE track_updated_at();
