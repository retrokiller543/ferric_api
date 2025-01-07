use crate::dto::{IntoDTO, UserDTOVecResponses};
use crate::error::ApiError;
use crate::repositories::users::UsersRepository;
use crate::repositories::Repository;
use crate::utils::api_scope;
use actix_helper_utils::generate_endpoint;
use actix_web::{web, HttpResponse};

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

        Ok(HttpResponse::Ok().json(users))
    }
}
