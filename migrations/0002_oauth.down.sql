-- Add down migration script here
-- Drop indexes for access tokens
DROP INDEX IF EXISTS idx_oauth_access_token_token;
DROP INDEX IF EXISTS idx_oauth_token_token;
DROP INDEX IF EXISTS idx_oauth_token_type;
DROP INDEX IF EXISTS idx_oauth_token_expiration;

-- Drop indexes for auth codes
DROP INDEX IF EXISTS idx_oauth_auth_code_code;

-- Drop indexes for clients
DROP INDEX IF EXISTS idx_oauth_client_client_id;

-- Drop access tokens table
DROP TABLE IF EXISTS oauth_token;

-- Drop authorization codes table
DROP TABLE IF EXISTS oauth_auth_code;

-- Drop clients table
DROP TABLE IF EXISTS oauth_client;

-- Drop Types
DROP TYPE IF EXISTS grant_type;
DROP TYPE IF EXISTS token_type;