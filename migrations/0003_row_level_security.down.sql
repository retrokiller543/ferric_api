-- Drop policies
DROP POLICY IF EXISTS users_self_access ON users;
DROP POLICY IF EXISTS oauth_token_self_access ON oauth_token;
DROP POLICY IF EXISTS oauth_client_read ON oauth_client;
DROP POLICY IF EXISTS oauth_client_modify ON oauth_client;
DROP POLICY IF EXISTS oauth_auth_code_self_access ON oauth_auth_code;

-- Drop functions
DROP FUNCTION IF EXISTS current_user_id();
DROP FUNCTION IF EXISTS is_system_context();
DROP FUNCTION IF EXISTS set_system_context(BOOLEAN);
DROP FUNCTION IF EXISTS set_user_context(UUID);
DROP FUNCTION IF EXISTS clear_user_context();

-- Disable RLS
ALTER TABLE users DISABLE ROW LEVEL SECURITY;
ALTER TABLE oauth_token DISABLE ROW LEVEL SECURITY;
ALTER TABLE oauth_client DISABLE ROW LEVEL SECURITY;
ALTER TABLE oauth_auth_code DISABLE ROW LEVEL SECURITY;