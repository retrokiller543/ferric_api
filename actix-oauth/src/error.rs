use crate::dto::oauth_error::Oauth2Error;
use actix_web::body::BoxBody;
use actix_web::http::{header, StatusCode};
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum Oauth2ErrorType {
    #[error("invalid_request")]
    InvalidRequest,
    #[error("invalid_grant")]
    InvalidGrant,
    #[error("unsupported_grant_type")]
    UnsupportedGrantType,
    #[error("invalid_scope")]
    InvalidScope,
    #[error("invalid_client")]
    InvalidClient,
    #[error("unauthorized_client")]
    UnauthorizedClient,
}

impl Oauth2ErrorType {
    fn get_description(&self) -> String {
        match self {
            Oauth2ErrorType::InvalidRequest => {
                "The request is missing a required parameter, includes an invalid parameter value, includes a parameter more than once, or is otherwise malformed.".to_string()
            }
            Oauth2ErrorType::InvalidGrant => {
                "The provided authorization grant is invalid, expired, revoked, or was issued to another client.".to_string()
            }
            Oauth2ErrorType::UnsupportedGrantType => {
                "The authorization grant type is not supported by the authorization server.".to_string()
            }
            Oauth2ErrorType::InvalidScope => {
                "The requested scope is invalid, unknown, malformed, or exceeds the scope granted by the resource owner.".to_string()
            }
            Oauth2ErrorType::InvalidClient => "Client authentication failed.".to_string(),
            Oauth2ErrorType::UnauthorizedClient => {
                "The client is not authorized to request a token using this method.".to_string()
            }
        }
    }
}

impl ResponseError for Oauth2ErrorType {
    fn status_code(&self) -> StatusCode {
        match self {
            Oauth2ErrorType::InvalidRequest => StatusCode::BAD_REQUEST,
            Oauth2ErrorType::InvalidGrant => StatusCode::BAD_REQUEST,
            Oauth2ErrorType::UnsupportedGrantType => StatusCode::BAD_REQUEST,
            Oauth2ErrorType::InvalidScope => StatusCode::BAD_REQUEST,
            Oauth2ErrorType::InvalidClient => StatusCode::UNAUTHORIZED,
            Oauth2ErrorType::UnauthorizedClient => StatusCode::FORBIDDEN,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let oauth_error = Oauth2Error {
            error: self.to_string(),
            error_description: self.get_description(),
        };

        error!("Error occurred when handling OAuth request: {oauth_error:?}");

        let json = serde_json::to_string(&oauth_error).unwrap_or_else(|_| "{\"error\":\"server_error\",\"error_description\":\"An internal server error occurred.\"}".to_string());

        HttpResponse::build(self.status_code())
            .insert_header((header::CONTENT_TYPE, mime::APPLICATION_JSON.to_string()))
            .body(json)
    }
}
