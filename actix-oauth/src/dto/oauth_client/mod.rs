pub mod create;
pub mod update;

use crate::types::{ClientId, ClientSecret, GrantType, RedirectUri, Scopes};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct OAuthClientDTO {
    pub client_id: ClientId,
    pub client_secret: ClientSecret,
    pub redirect_uri: RedirectUri,
    pub grant_types: Vec<GrantType>,
    pub scopes: Scopes,
    pub created_at: NaiveDateTime,
}
