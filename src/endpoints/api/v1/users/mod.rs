//! TODO: Most endpoints have no authentication in this module, this needs to be changed

use crate::endpoints::api::v1::users::get::users_get_service;
use crate::utils::api_scope;

mod delete;
mod get;
mod post;

api_scope! {
    pub(super) Users = "/users";

    services: [users_get_service];
    paths: [post::create_user, delete::delete_user];
    docs: {
        extra_paths: [get::get_users, get::by_id::get_user_by_id];
        schemas: [crate::dto::UserDTO];
        responses: [crate::dto::UserDTO, crate::dto::UserDTOVecResponses];
    }
}
