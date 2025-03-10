use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx_utils::sql_filter;
use sqlx_utils::traits::Model;
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

sql_filter! {
    #[derive(Default, Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
    pub struct UserFilter {
        SELECT * FROM users
        WHERE
            ?id = i64
            AND ext_id = Uuid
            AND ?username = String
            OR ?email = String
            AND ?created_at as created_before < NaiveDateTime
            AND ?created_at as created_after > NaiveDateTime
            AND ?updated_at as updated_before < NaiveDateTime
            AND ?updated_at as updated_after > NaiveDateTime
    }
}
