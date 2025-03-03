//! Error types and handling for OAuth2 requests.
//!
//! This module provides standardized error types according to the OAuth2 specification
//! (RFC 6749, Section 5.2), with appropriate HTTP status codes and JSON responses.

use crate::dto::oauth_error::Oauth2Error;
use actix_web::body::BoxBody;
use actix_web::http::{StatusCode, header};
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use tracing::error;

/// Represents all possible OAuth2 error types according to the OAuth2 specification.
///
/// These error types correspond to the standard OAuth2 error values as defined in
/// RFC 6749, Section 5.2, plus additional error types for server errors.
///
/// # Examples
///
/// ```
/// use actix_oauth::error::Oauth2ErrorType;
///
/// // Return an invalid grant error
/// fn validate_token(token: &str) -> Result<(), Oauth2ErrorType> {
///     if token.len() < 10 {
///         return Err(Oauth2ErrorType::InvalidGrant);
///     }
///     Ok(())
/// }
/// ```
#[derive(Error, Debug)]
pub enum Oauth2ErrorType {
    /// The request is missing a required parameter, includes an invalid parameter value,
    /// includes a parameter more than once, or is otherwise malformed.
    #[error("invalid_request")]
    InvalidRequest,
    /// The provided authorization grant (e.g., authorization code, resource owner credentials)
    /// or refresh token is invalid, expired, revoked, does not match the redirection URI used
    /// in the authorization request, or was issued to another client.
    #[error("invalid_grant")]
    InvalidGrant,
    /// The authorization grant type is not supported by the authorization server.
    #[error("unsupported_grant_type")]
    UnsupportedGrantType,
    /// The requested scope is invalid, unknown, malformed, or exceeds the scope
    /// granted by the resource owner.
    #[error("invalid_scope")]
    InvalidScope,
    /// Client authentication failed (e.g., unknown client, no client authentication included,
    /// or unsupported authentication method).
    #[error("invalid_client")]
    InvalidClient,
    /// The authenticated client is not authorized to use this authorization grant type.
    #[error("unauthorized_client")]
    UnauthorizedClient,
    /// The authorization server encountered an unexpected condition that prevented it
    /// from fulfilling the request.
    #[error("server_error")]
    ServerError,
    /// An internal error occurred that doesn't map to a standard OAuth2 error.
    #[error("internal_error")]
    InternalError(String),
}

impl Oauth2ErrorType {
    /// Returns a human-readable description of the error.
    ///
    /// These descriptions are based on the OAuth2 specification and provide
    /// more detailed information about the error.
    ///
    /// # Returns
    ///
    /// A string containing the error description.
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
            Oauth2ErrorType::ServerError => "An internal server error has occurred".to_string(),
            Oauth2ErrorType::InternalError(s) => s.to_string()
        }
    }
}

impl ResponseError for Oauth2ErrorType {
    /// Maps OAuth2 errors to appropriate HTTP status codes.
    ///
    /// This follows OAuth2 best practices for error responses:
    /// - Most client errors (invalid request, grant, etc.) return 400 Bad Request
    /// - Authentication failures return 401 Unauthorized
    /// - Authorization failures return 403 Forbidden
    /// - Server errors return 500 Internal Server Error
    ///
    /// # Returns
    ///
    /// The HTTP status code corresponding to this error.
    fn status_code(&self) -> StatusCode {
        match self {
            Oauth2ErrorType::InvalidRequest => StatusCode::BAD_REQUEST,
            Oauth2ErrorType::InvalidGrant => StatusCode::BAD_REQUEST,
            Oauth2ErrorType::UnsupportedGrantType => StatusCode::BAD_REQUEST,
            Oauth2ErrorType::InvalidScope => StatusCode::BAD_REQUEST,
            Oauth2ErrorType::InvalidClient => StatusCode::UNAUTHORIZED,
            Oauth2ErrorType::UnauthorizedClient => StatusCode::FORBIDDEN,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Builds a properly formatted OAuth2 error response.
    ///
    /// The response includes:
    /// - Appropriate HTTP status code
    /// - JSON content type header
    /// - JSON body with error and error_description fields
    ///
    /// This method also logs the error at the error level.
    ///
    /// # Returns
    ///
    /// An HTTP response containing the error information.
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
