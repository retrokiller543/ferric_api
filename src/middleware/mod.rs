pub mod rls_context;

use crate::models::oauth_token::TokenType;
use crate::prelude::*;
use crate::repositories::oauth_token::OauthTokenRepository;
use crate::repositories::users::UsersRepository;
use crate::utils::header::extract_bearer_token;
use crate::utils::middleware_macros::define_middleware;
use actix_oauth::types::AccessToken;
use actix_web::dev::ServiceRequest;
use actix_web::http::StatusCode;
use actix_web::http::header::AUTHORIZATION;
use actix_web::{HttpMessage, HttpResponse, ResponseError};
use thiserror::Error;
use tracing::warn;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Missing authorization header")]
    MissingAuth,
    #[error("Invalid access token")]
    InvalidToken,
    #[error("Invalid token type")]
    InvalidTokenType,
    #[error("User not found")]
    UserNotFound,
    #[error("Internal server error")]
    InternalError,
}

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::MissingAuth => StatusCode::UNAUTHORIZED,
            AuthError::InvalidToken => StatusCode::UNAUTHORIZED,
            AuthError::InvalidTokenType => StatusCode::UNAUTHORIZED,
            AuthError::UserNotFound => StatusCode::UNAUTHORIZED,
            AuthError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(serde_json::json!({
            "error": self.to_string(),
            "status": self.status_code().as_u16()
        }))
    }
}

define_middleware! {
    #[derive(Debug)]
    pub struct AuthMiddleware = "AuthMiddleware" {
        token_repo: OauthTokenRepository,
        user_repo: UsersRepository,
    },

    pub struct AuthMiddlewareService;

    |service: AuthMiddlewareService<S>, req: ServiceRequest| async move {
        {
            let headers = req.headers();

            let auth_header = headers
                .get(AUTHORIZATION)
                .ok_or(AuthError::MissingAuth)?;

            let mut token = auth_header
                .to_str()
                .map_err(|error| {
                    warn!(?error, "Failed to convert auth header to string, header might contain non ASCII characters");
                    AuthError::InvalidToken
                })?;
            token = extract_bearer_token(token).ok_or(AuthError::InvalidToken)?;

            let context = UserContext::System;
            let token = service.token_repo.get_by_token_with_context(token, &context).await.map_err(|_| AuthError::InvalidToken)?;

            if token.token_type != TokenType::Access {
                return Err(AuthError::InvalidTokenType.into())
            }

            let user = service.user_repo
                .get_by_ext_id_with_context(token.user_ext_id, &context)
                .await
                .map_err(|_| AuthError::InternalError)?
                .ok_or(AuthError::UserNotFound)?;

            let mut ext = req.extensions_mut();
            ext.insert(AccessToken::new(token.token.clone()));
            ext.insert(token);
            ext.insert(user);
        }

        // Continue with the request
        service.service.call(req).await
    }
}
