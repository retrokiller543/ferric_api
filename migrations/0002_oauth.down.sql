-- Add down migration script here
DROP INDEX IF EXISTS idx_oauth_token_user_id;
DROP INDEX IF EXISTS idx_oauth_token_expires_at_valid;
DROP INDEX IF EXISTS idx_oauth_auth_code_user_id;
DROP INDEX IF EXISTS idx_oauth_auth_code_expires_at_valid;

DROP TABLE IF EXISTS oauth_token CASCADE;
DROP TABLE IF EXISTS oauth_auth_code CASCADE;
DROP TABLE IF EXISTS oauth_client CASCADE;
DROP TYPE IF EXISTS grant_type CASCADE;
DROP TYPE IF EXISTS token_type CASCADE;
