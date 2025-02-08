use crate::models::oauth_token::TokenType;
use crate::repositories::oauth_token::{OauthTokenFilter, OauthTokenRepository};
use crate::repositories::users::UsersRepository;
use crate::utils::middleware_macros::define_middleware;
use actix_web::dev::ServiceRequest;
use actix_web::http::header::AUTHORIZATION;
use actix_web::http::StatusCode;
use actix_web::{HttpMessage, HttpResponse, ResponseError};
use sqlx_utils::traits::Repository;
use thiserror::Error;

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
    pub struct AuthMiddleware {
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

            let token = auth_header
                .to_str()
                .map_err(|_| AuthError::InvalidToken)?;

            let token_res = service.token_repo.get_by_filter(OauthTokenFilter::new().token(token)).await?;
            let token_model = token_res.first().cloned();

            let token = token_model.ok_or(AuthError::InvalidToken)?;

            if token.token_type != TokenType::Access {
                return Err(AuthError::InvalidTokenType.into())
            }

            let user = service.user_repo
                .get_by_id(token.user_ext_id)
                .await
                .map_err(|_| AuthError::InternalError)?
                .ok_or(AuthError::UserNotFound)?;

            let mut ext = req.extensions_mut();
            ext.insert(token);
            ext.insert(user);
        }

        // Continue with the request
        service.service.call(req).await
    }
}
