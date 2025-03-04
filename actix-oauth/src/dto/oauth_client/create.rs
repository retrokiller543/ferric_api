use crate::types::{GrantType, RedirectUri, Scopes};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// DTO for creating a new OAuth2 Client
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, ToSchema, Validate)]
pub struct OAuthCreateClientDTO {
    /// The predefined redirect uri that is supported by the client
    #[validate(url)]
    pub redirect_uri: RedirectUri,
    /// What grant types that the client supports
    pub grant_types: Vec<GrantType>,
    /// Scopes the client is allowed to use
    pub scopes: Scopes,
}
