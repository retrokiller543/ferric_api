use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx_utils::traits::Model;
use std::fmt::{Display, Formatter};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(
    Default,
    Debug,
    Clone,
    Copy,
    Hash,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Serialize,
    Deserialize,
    sqlx::Type,
    ToSchema,
)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "token_type", rename_all = "snake_case")]
pub enum TokenType {
    #[default]
    Access,
    Refresh,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Access => write!(f, "access"),
            Self::Refresh => write!(f, "refresh"),
        }
    }
}

#[derive(
    Default, Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, FromRow, Serialize, Deserialize,
)]
pub struct OAuthToken {
    pub(crate) id: Option<i64>,
    pub(crate) token: String,
    pub(crate) client_id: Option<i64>,
    pub(crate) user_ext_id: Uuid,
    pub(crate) token_type: TokenType,
    pub(crate) scopes: Vec<String>,
    pub(crate) expires_at: NaiveDateTime,
    pub(crate) created_at: Option<NaiveDateTime>,
}

impl OAuthToken {
    pub(crate) fn new(
        token: String,
        user_ext_id: Uuid,
        token_type: TokenType,
        expires_at: NaiveDateTime,
    ) -> Self {
        Self {
            id: None,
            token,
            client_id: None,
            user_ext_id,
            token_type,
            scopes: Vec::new(),
            expires_at,
            created_at: None,
        }
    }
}

impl Model for OAuthToken {
    type Id = i64;

    fn get_id(&self) -> Option<Self::Id> {
        self.id
    }
}
