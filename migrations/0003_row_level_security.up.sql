-- Enable Row Level Security on tables
ALTER TABLE users ENABLE ROW LEVEL SECURITY;
ALTER TABLE oauth_token ENABLE ROW LEVEL SECURITY;
ALTER TABLE oauth_client ENABLE ROW LEVEL SECURITY;
ALTER TABLE oauth_auth_code ENABLE ROW LEVEL SECURITY;

ALTER TABLE users FORCE ROW LEVEL SECURITY;
ALTER TABLE oauth_token FORCE ROW LEVEL SECURITY;
ALTER TABLE oauth_client FORCE ROW LEVEL SECURITY;
ALTER TABLE oauth_auth_code FORCE ROW LEVEL SECURITY;

-- Create a function to set user context
CREATE OR REPLACE FUNCTION set_user_context(p_user_id UUID)
    RETURNS VOID AS $$
BEGIN
    PERFORM set_config('app.current_user_id', p_user_id::TEXT, FALSE);
END;
$$ LANGUAGE plpgsql;

-- Create a function to clear user context
CREATE OR REPLACE FUNCTION clear_user_context()
    RETURNS VOID AS $$
BEGIN
    PERFORM set_config('app.current_user_id', NULL, FALSE);
END;
$$ LANGUAGE plpgsql;

-- Create a helper function to get current user ID
CREATE OR REPLACE FUNCTION current_user_id()
    RETURNS UUID AS $$
BEGIN
    RETURN nullif(current_setting('app.current_user_id', TRUE), '')::UUID;
END;
$$ LANGUAGE plpgsql;

-- Create a function to check if in system context
CREATE OR REPLACE FUNCTION is_system_context()
    RETURNS BOOLEAN AS $$
BEGIN
    RETURN nullif(current_setting('app.is_system_context', TRUE), '') = 'true';
END;
$$ LANGUAGE plpgsql;

-- Create a function to set system context (bypasses RLS)
CREATE OR REPLACE FUNCTION set_system_context(enabled BOOLEAN)
    RETURNS VOID AS $$
BEGIN
    PERFORM set_config('app.is_system_context', enabled::TEXT, FALSE);
END;
$$ LANGUAGE plpgsql;

-- Users table policies
-- Users can only see and modify their own user data
CREATE POLICY users_self_access ON users
    FOR ALL
    USING (ext_id = current_user_id() OR is_system_context());

-- OAuth token policies
-- Users can only see and modify their own tokens
CREATE POLICY oauth_token_self_access ON oauth_token
    FOR ALL
    USING (user_ext_id = current_user_id() OR is_system_context());

-- OAuth client policies
-- For now, clients are globally accessible for reading
CREATE POLICY oauth_client_read ON oauth_client
    FOR SELECT
    USING (TRUE);

-- Client owners can modify their clients
CREATE POLICY oauth_client_modify ON oauth_client
    FOR ALL
    USING (
    id IN (
        SELECT client_id FROM oauth_token
        WHERE user_ext_id = current_user_id()
    ) OR is_system_context()
    );

-- OAuth auth code policies
-- Users can only see and modify their own auth codes
CREATE POLICY oauth_auth_code_self_access ON oauth_auth_code
    FOR ALL
    USING (user_ext_id = current_user_id() OR is_system_context());