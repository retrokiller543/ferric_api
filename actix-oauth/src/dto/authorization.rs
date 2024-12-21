use crate::types::{ClientId, RedirectUri, Scopes};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

/// Represents the `response_type` parameter in OAuth2 Authorization Endpoint requests.
#[derive(Serialize, Deserialize, Debug, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ResponseType {
    /// Used for Authorization Code Flow
    Code,
    /// Used for Hybrid Flow (OIDC)
    Token,
    /// Used for Hybrid Flow with both code and id_token
    #[serde(rename = "code id_token")]
    CodeIdToken,
    /// Used for Hybrid Flow with code, id_token, and token
    #[serde(rename = "code id_token token")]
    CodeIdTokenToken,
}

#[derive(Deserialize, Debug, ToSchema, IntoParams)]
#[into_params(style = Form, parameter_in = Query)]
pub struct AuthorizationRequest {
    /// OAuth2 response type (e.g., `code`, `token`, or hybrid combinations)
    #[schema(example = "code")]
    pub response_type: ResponseType,

    /// Client ID requesting authorization
    pub client_id: ClientId,

    /// Redirect URI to send the authorization code or token
    pub redirect_uri: RedirectUri,

    /// Optional scope for requested permissions
    pub scope: Option<Scopes>,

    /// Optional state to prevent CSRF attacks
    #[schema(example = "random_state_value")]
    pub state: Option<String>,
}
