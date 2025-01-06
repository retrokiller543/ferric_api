use crate::endpoints::api::v1::users::get::users_get_service;
use crate::openapi::NormalizePath;
use actix_web::dev::HttpServiceFactory;
use actix_web::web;
use utoipa::OpenApi;

mod delete;
mod get;
mod post;

#[derive(OpenApi)]
#[openapi(
    paths(post::create_user),
    components(schemas(crate::dto::user::create::UserCreateDTO), responses()),
    modifiers(&NormalizePath)
)]
pub(super) struct UsersAPI;

pub(crate) fn users_service() -> impl HttpServiceFactory {
    web::scope("/users")
        .service(users_get_service())
        .service(post::create_user)
        .service(delete::delete_user)
}
