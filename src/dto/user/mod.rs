pub(crate) mod create;

use actix_oauth::impl_responder;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::{IntoResponses, ToResponse, ToSchema};
use uuid::Uuid;
use validator::Validate;

#[derive(
    Default,
    Debug,
    Clone,
    Hash,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
    ToSchema,
    ToResponse,
    Validate,
)]
#[response(examples(
        ("Successful" = (value = json!({
                "id": "068cd24f-730f-451b-b4c7-e8fd81637701",
                "username": "testuser",
                "email": "test@test.com",
                "created_at": "2025-01-06T16:00:45.770588",
                "updated_at": "2025-01-06T16:00:45.770588"
            }), description = "Successfully found user"
        ))
))]
pub(crate) struct UserDTO {
    pub(crate) id: Uuid,
    pub(crate) username: String,
    #[validate(email)]
    pub(crate) email: String,
    pub(crate) created_at: Option<NaiveDateTime>,
    pub(crate) updated_at: Option<NaiveDateTime>,
}

impl_responder!(UserDTO);

#[derive(ToResponse)]
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
pub(crate) struct UserDTOVecResponses(#[allow(dead_code)] Vec<UserDTO>);
