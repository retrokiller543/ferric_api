pub mod create;
pub mod update;

pub use {create::*, update::*};

use crate::impl_responder;
use crate::types::{ClientId, ClientSecret, GrantType, RedirectUri, Scopes};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};
use validator::Validate;

#[derive(
    Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize, ToSchema, Validate, ToResponse,
)]
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
