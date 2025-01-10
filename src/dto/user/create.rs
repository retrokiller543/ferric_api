use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// User creation DTO object
///
/// The required input we need from a user to creat a new user
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
pub(crate) struct UserCreateDTO {
    pub(crate) username: String,
    #[validate(email)]
    pub(crate) email: String,
    pub(crate) password: String,
}
