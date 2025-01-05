-- Add down migration script here
DROP INDEX IF EXISTS idx_users_username;
DROP INDEX IF EXISTS idx_users_email;
DROP INDEX IF EXISTS idx_users_ext_id;

DROP TABLE IF EXISTS users;