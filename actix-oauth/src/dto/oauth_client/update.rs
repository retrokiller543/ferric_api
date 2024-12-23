use crate::types::{GrantType, RedirectUri, Scopes};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct OAuthUpdateClientDTO {
    pub redirect_uri: RedirectUri,
    pub grant_types: Vec<GrantType>,
    pub scopes: Scopes,
}
