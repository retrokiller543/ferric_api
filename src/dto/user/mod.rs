pub(crate) mod create;

use actix_oauth::impl_responder;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
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
    Validate,
)]
pub(crate) struct UserDTO {
    pub(crate) id: Uuid,
    pub(crate) username: String,
    #[validate(email)]
    pub(crate) email: String,
    pub(crate) created_at: Option<NaiveDateTime>,
    pub(crate) updated_at: Option<NaiveDateTime>,
}

impl_responder!(UserDTO);
