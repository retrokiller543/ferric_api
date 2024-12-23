use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, sqlx::Type, ToSchema,
)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "grant_type", rename_all = "snake_case")]
pub enum GrantType {
    AuthorizationCode,
    ClientCredentials,
    RefreshToken,
}
