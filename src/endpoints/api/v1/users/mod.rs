use crate::endpoints::api::v1::users::get::users_get_service;
use actix_web::dev::HttpServiceFactory;
use actix_web::web;

mod delete;
mod get;
mod post;

pub(crate) fn users_service() -> impl HttpServiceFactory {
    web::scope("/users")
        .service(users_get_service())
        .service(post::create_user)
        .service(delete::delete_user)
}
