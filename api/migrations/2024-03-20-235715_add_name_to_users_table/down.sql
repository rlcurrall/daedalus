/*
|-------------------------------------------------------------------------------
| Remove Name from Users Table
|-------------------------------------------------------------------------------
|
| This migration removes the name column from the users table.
|
| @date 2024-03-20
| @author Robb Currall <robb@currall.net>
|
*/

ALTER TABLE users
    DROP COLUMN name;
