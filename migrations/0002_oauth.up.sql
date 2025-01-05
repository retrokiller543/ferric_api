CREATE TYPE grant_type AS ENUM('authorization_code', 'client_credentials', 'refresh_token');
CREATE TYPE token_type AS ENUM('access', 'refresh');

CREATE TABLE oauth_client (
    id BIGSERIAL PRIMARY KEY,
    client_id TEXT NOT NULL UNIQUE,
    client_secret TEXT NOT NULL, -- can store hashed secrets here
    redirect_uri TEXT NOT NULL,
    grant_types grant_type[] NOT NULL,
    scopes TEXT[] NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE oauth_auth_code (
    id BIGSERIAL PRIMARY KEY,
    code TEXT NOT NULL UNIQUE, -- can also store a hash here if desired
    client_id BIGINT NOT NULL REFERENCES oauth_client(id) ON DELETE CASCADE,
    user_ext_id UUID NOT NULL REFERENCES users(ext_id) ON DELETE CASCADE,
    redirect_uri TEXT NOT NULL,
    scopes TEXT[] NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE oauth_token (
    id BIGSERIAL PRIMARY KEY,
    token TEXT NOT NULL UNIQUE, -- can also store a hash here if desired
    client_id BIGINT REFERENCES oauth_client(id) ON DELETE CASCADE,
    user_ext_id UUID NOT NULL REFERENCES users(ext_id) ON DELETE CASCADE,
    token_type token_type NOT NULL,
    scopes TEXT[] NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_oauth_token_user_ext_id ON oauth_token(user_ext_id);
CREATE INDEX idx_oauth_token_expires_at ON oauth_token(expires_at);


CREATE INDEX idx_oauth_auth_code_user_ext_id ON oauth_auth_code(user_ext_id);
CREATE INDEX idx_oauth_auth_code_expires_at ON oauth_auth_code(expires_at);
