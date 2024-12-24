pub mod create;
pub mod update;

pub use {create::*, update::*};

use crate::impl_responder;
use crate::types::{ClientId, ClientSecret, GrantType, RedirectUri, Scopes};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, ToSchema, Validate)]
pub struct OAuthClientDTO {
    pub client_id: ClientId,
    pub client_secret: ClientSecret,
    #[validate(url)]
    pub redirect_uri: RedirectUri,
    pub grant_types: Vec<GrantType>,
    pub scopes: Scopes,
    pub created_at: NaiveDateTime,
}

impl_responder!(OAuthClientDTO);
