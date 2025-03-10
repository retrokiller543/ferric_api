use crate::dto::UserDTOCollection;
use crate::error::ApiError;
use crate::middleware::AuthMiddleware;
use crate::middleware::rls_context::RlsContextMiddleware;
use crate::prelude::*;
use crate::repositories::oauth_token::OAUTH_TOKEN_REPOSITORY;
use crate::repositories::users::{USERS_REPOSITORY, UsersRepository};
use crate::traits::into_dto::IntoDTO;
use crate::utils::api_scope;
use actix_helper_utils::generate_endpoint;
use actix_web::web;

pub(crate) mod by_id;

async fn rls_context_middleware() -> ApiResult<RlsContextMiddleware> {
    Ok(RlsContextMiddleware::new())
}

async fn auth_middleware() -> ApiResult<AuthMiddleware> {
    let token_repo = *OAUTH_TOKEN_REPOSITORY;
    let user_repo = *USERS_REPOSITORY;

    Ok(AuthMiddleware::new(token_repo, user_repo))
}

api_scope! {
    pub(super) users_get = "";

    guard: Get;
    middleware: [rls_context: rls_context_middleware, auth: auth_middleware];
    paths: [get_users, by_id::get_user_by_id];
}

generate_endpoint! {
    /// Gets all users that are registered.
    #[tracing::instrument(skip(repo))]
    fn get_users;
    method: get;
    path: "";
    return_type: UserDTOCollection;
    error: ApiError;
    docs: {
        tag: "user",
        context_path: "/users",
        responses: {
            (status = 200, description = "", body = UserDTOCollection)
        },
        security: [
            ("bearer_token" = [])
        ]
    }
    params: {
        repo: web::Data<UsersRepository>,
        user_context: UserContext,
    }
    {
        let users = repo.get_all_with_context(&user_context).await?.into_dto();

        Ok(users)
    }
}
