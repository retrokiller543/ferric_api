mod_def! {
    pub(crate) mod create;
}

use crate::dto;
use crate::mod_def;
use crate::models::oauth_client::OAuthClient;
use crate::models::user::User;
use crate::traits::{FromModel, IntoDTO};
use actix_oauth::dto::{OAuthClientDTO, OAuthClientDTOCollection};
use actix_oauth::impl_responder;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::{IntoResponses, ToResponse, ToSchema};
use uuid::Uuid;
use validator::Validate;

dto! {
    /// Represents a user returned by the server.
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
    pub(crate) struct UserDTO => User {
        pub(crate) id: Uuid,
        pub(crate) username: String,
        #[validate(email)]
        pub(crate) email: String,
        pub(crate) created_at: Option<NaiveDateTime>,
        pub(crate) updated_at: Option<NaiveDateTime>
    }

    fn from_model(model: User) -> Self {
        Self {
            id: model.ext_id.unwrap(),
            username: model.username,
            email: model.email,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

/// A list of `UserDTO` objects.
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
