use crate::dto::{UserDTOCollection, UserDTOVecResponses};
use crate::error::ApiError;
use crate::repositories::users::UsersRepository;
use crate::traits::into_dto::IntoDTO;
use crate::traits::repository::Repository;
use crate::utils::api_scope;
use actix_helper_utils::generate_endpoint;
use actix_web::web;

pub(crate) mod by_id;

api_scope! {
    pub(super) users_get = "";

    guard: Get;
    paths: [get_users, by_id::get_user_by_id];
}

generate_endpoint! {
    fn get_users;
    method: get;
    path: "";
    return_type: UserDTOCollection;
    error: ApiError;
    docs: {
        tag: "user",
        context_path: "/users",
        responses: {
            (status = 200, response = UserDTOVecResponses)
        }
    }
    params: {
        repo: web::Data<UsersRepository>
    }
    {
        let users = repo.get_all().await?.into_dto();

        Ok(users)
    }
}
