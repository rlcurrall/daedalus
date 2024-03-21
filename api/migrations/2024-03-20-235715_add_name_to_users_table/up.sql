/*
|-------------------------------------------------------------------------------
| Add Name to Users Table
|-------------------------------------------------------------------------------
|
| This migration adds a name column to the users table.
|
| @date 2024-03-20
| @author Robb Currall <robb@currall.net>
|
*/

ALTER TABLE users
    ADD COLUMN name VARCHAR(255);
