/*
|-------------------------------------------------------------------------------
| Drop tenants table
|-------------------------------------------------------------------------------
|
| This migration drops the tenants table.
|
| @date 2023-12-28
| @author Robb Currall <robb@currall.net>
|
*/

-- Drop the trigger on the tenants table
DROP TRIGGER IF EXISTS update_tenants_updated_at ON tenants;

-- Drop the tenants table
DROP TABLE IF EXISTS tenants;

-- Drop the track_updated_at function
DROP FUNCTION IF EXISTS track_updated_at();
