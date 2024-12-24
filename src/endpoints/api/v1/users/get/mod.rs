use crate::dto::IntoDTO;
use crate::error::ApiError;
use crate::repositories::users::UsersRepository;
use crate::repositories::Repository;
use actix_helper_utils::generate_endpoint;
use actix_web::dev::HttpServiceFactory;
use actix_web::{web, HttpResponse};

pub(crate) mod by_id;

pub(crate) fn users_get_service() -> impl HttpServiceFactory {
    web::scope("")
        .guard(actix_web::guard::Get())
        .service(get_users)
        .service(by_id::get_user_by_id)
}

generate_endpoint! {
    fn get_users;
    method: get;
    path: "";
    error: ApiError;
    params: {
        repo: web::Data<UsersRepository>
    }
    {
        let users = repo.get_all().await?.into_dto();

        Ok(HttpResponse::Ok().json(users))
    }
}
