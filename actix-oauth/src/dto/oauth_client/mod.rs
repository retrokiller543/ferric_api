pub mod create;
pub mod update;

pub use {create::*, update::*};

use crate::impl_responder;
use crate::types::{ClientId, ClientSecret, GrantType, RedirectUri, Scopes};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};
use validator::Validate;

/// Represents a OAuth2 client returned by the Server.
#[derive(
    Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, ToSchema, Validate, ToResponse,
)]
pub struct OAuthClientDTO {
    /// The ID that should be used when authenticating.
    pub client_id: ClientId,
    /// The secret that should be used when authenticating.
    pub client_secret: ClientSecret,
    /// The predefined redirect uri that is supported by the client
    #[validate(url)]
    pub redirect_uri: RedirectUri,
    /// What grant types that the client supports
    pub grant_types: Vec<GrantType>,
    /// Scopes the client is allowed to use
    pub scopes: Scopes,
    /// When the client was created
    pub created_at: NaiveDateTime,
}

impl_responder!(OAuthClientDTO);

/*#[derive(ToResponse)]
#[response(examples(
        ("Successful" = (value = json!([{
                "id": "068cd24f-730f-451b-b4c7-e8fd81637701",
                "username": "testuser",
                "email": "test@test.com",
                "created_at": "2025-01-06T16:00:45.770588",
                "updated_at": "2025-01-06T16:00:45.770588"
            }]), description = "Successfully found users"
        ))
))]
pub(crate) struct UserDTOVecResponses(#[allow(dead_code)] OAuthClientDTOCollection);*/
