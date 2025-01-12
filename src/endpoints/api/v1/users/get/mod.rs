use crate::dto::{UserDTOCollection, UserDTOVecResponses};
use crate::error::ApiError;
use crate::middleware::AuthMiddleware;
use crate::repositories::oauth_token::get_oauth_token_repository;
use crate::repositories::users::{get_users_repository, UsersRepository};
use crate::traits::into_dto::IntoDTO;
use crate::traits::repository::Repository;
use crate::utils::api_scope;
use actix_helper_utils::generate_endpoint;
use actix_web::web;

pub(crate) mod by_id;

api_scope! {
    pub(super) users_get = "";

    guard: Get;
    middleware: [auth: || async {
        let token_repo = get_oauth_token_repository().await?;
        let user_repo = get_users_repository().await?;

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
