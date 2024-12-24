use crate::impl_responder;
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::json;
use utoipa::{IntoResponses, ToResponse, ToSchema};

#[derive(
    Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, ToSchema, ToResponse,
)]
pub struct Oauth2Error {
    /// A single ASCII string from a predefined set of error codes.
    pub(crate) error: String,
    /// A human-readable ASCII string providing additional information about the error.
    pub(crate) error_description: String,
}

impl_responder!(Oauth2Error);

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoResponses)]
pub enum Oauth2ErrorResponses {
    /// Error a bad request was sent to the server.
    #[response(status = 400,
        examples(
            ("invalid_request" = (description = "The request is missing a required parameter, includes an invalid parameter value, includes a parameter more than once, or is otherwise malformed.", value = json!({"error": "invalid_request", "error_description": "The request is missing a required parameter or is malformed."}))),
            ("invalid_grant" = (description = "The provided authorization grant (e.g., authorization code, resource owner credentials) or refresh token is invalid, expired, revoked, or was issued to another client.", value = json!({"error": "invalid_grant", "error_description": "The authorization code is invalid or expired."}))),
            ("unsupported_grant_type" = (description = "The authorization grant type is not supported by the authorization server.", value = json!({"error": "unsupported_grant_type", "error_description": "The authorization server does not support the requested grant type."}))),
            ("invalid_scope" = (description = "The requested scope is invalid, unknown, malformed, or exceeds the scope granted by the resource owner.", value = json!({"error": "invalid_scope", "error_description": "The requested scope is invalid, unknown, or exceeds the granted scope."}))),
        ))
    ]
    BadRequest(Oauth2Error),
    /// Error: `invalid_client`, HTTP Status: 401 Unauthorized
    #[response(status = 401, example = json!({
        "error": "invalid_client",
        "error_description": "Client authentication failed."
    }))]
    InvalidClient(Oauth2Error),

    /// Error: `unauthorized_client`, HTTP Status: 403 Forbidden
    #[response(status = 403, example = json!({
        "error": "unauthorized_client",
        "error_description": "The client is not authorized to request a token using this method."
    }))]
    UnauthorizedClient(Oauth2Error),

    #[response(status = 500, example = json!({
        "error": "server_error",
        "error_description": "An internal server error occurred."
    }))]
    ServerError(Oauth2Error),
}
