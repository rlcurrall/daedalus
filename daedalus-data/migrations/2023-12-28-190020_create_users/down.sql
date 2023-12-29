-- -----------------------------------------------------------------------------
-- Revert: Create users table
--
-- @date 2023-12-28
-- @author Robb Currall <robb@currall.net>
-- -----------------------------------------------------------------------------

-- Drop the trigger on the users table
DROP TRIGGER IF EXISTS update_users_updated_at ON users;

-- Drop the users table
DROP TABLE IF EXISTS users;
