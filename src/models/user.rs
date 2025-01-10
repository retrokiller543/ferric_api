use crate::traits::model::Model;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(
    Default, Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, FromRow, Serialize, Deserialize,
)]
pub(crate) struct User {
    pub(crate) id: Option<i64>,
    pub(crate) ext_id: Option<Uuid>,
    pub(crate) username: String,
    pub(crate) password_hash: String,
    pub(crate) email: String,
    pub(crate) created_at: Option<NaiveDateTime>,
    pub(crate) updated_at: Option<NaiveDateTime>,
}

impl Model for User {
    type Id = Uuid;

    fn get_id(&self) -> Option<Self::Id> {
        self.ext_id
    }
}
