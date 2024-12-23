-- Add up migration script here
CREATE TYPE grant_type AS ENUM('authorization_code', 'client_credentials', 'refresh_token');
CREATE TYPE token_type AS ENUM('access', 'refresh');

-- Table to store OAuth clients
CREATE TABLE oauth_client (
    client_id TEXT PRIMARY KEY,
    client_secret TEXT NOT NULL,
    redirect_uri TEXT NOT NULL,
    grant_types grant_type[] NOT NULL,
    scopes TEXT[] NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Table to store authorization codes
CREATE TABLE oauth_auth_code (
    code TEXT PRIMARY KEY,
    client_id TEXT NOT NULL REFERENCES oauth_client(client_id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    redirect_uri TEXT NOT NULL,
    scopes TEXT[] NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Table to store access and refresh tokens
CREATE TABLE oauth_token (
    token TEXT PRIMARY KEY,
    client_id TEXT NOT NULL REFERENCES oauth_client(client_id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_type token_type NOT NULL,
    scopes TEXT[] NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for token management
CREATE INDEX idx_oauth_token_token ON oauth_token(token);
CREATE INDEX idx_oauth_token_type ON oauth_token(token_type);
CREATE INDEX idx_oauth_token_expiration ON oauth_token(expires_at);
CREATE INDEX idx_oauth_client_client_id ON oauth_client(client_id);
CREATE INDEX idx_oauth_auth_code_code ON oauth_auth_code(code);

CREATE INDEX idx_oauth_auth_code_user_id ON oauth_auth_code(user_id);
CREATE INDEX idx_oauth_token_user_id ON oauth_token(user_id);

