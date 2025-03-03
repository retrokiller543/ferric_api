use crate::dto::{UserDTOCollection, UserDTOVecResponses};
use crate::error::ApiError;
use crate::middleware::AuthMiddleware;
use crate::repositories::oauth_token::OAUTH_TOKEN_REPOSITORY;
use crate::repositories::users::{USERS_REPOSITORY, UsersRepository};
use crate::traits::into_dto::IntoDTO;
use crate::utils::api_scope;
use actix_helper_utils::generate_endpoint;
use actix_web::web;
use sqlx_utils::traits::Repository;

pub(crate) mod by_id;

api_scope! {
    pub(super) users_get = "";

    guard: Get;
    middleware: [auth: || async {
        let token_repo = *OAUTH_TOKEN_REPOSITORY;
        let user_repo = *USERS_REPOSITORY;

        Ok::<_, ApiError>(AuthMiddleware::new(token_repo, user_repo))
    }];
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
