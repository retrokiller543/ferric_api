use crate::types::{GrantType, RedirectUri, Scopes};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, ToSchema, Validate)]
pub struct OAuthCreateClientDTO {
    #[validate(url)]
    pub redirect_uri: RedirectUri,
    pub grant_types: Vec<GrantType>,
    pub scopes: Scopes,
}
