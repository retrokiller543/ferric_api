//! All endpoints that this API serves, the structure of this module is built as closely as possible to the actual endpoints' path.
//!
//! # Example
//!
//! If we have the endpoint `GET /api/v1/users` then the file path will be, relative to the endpoints module, `api/v1/users`.
//! If we add another endpoint with the same path but different HTTP method, for example we also have a path `POST /api/v1/users`
//! then we have the file paths `api/v1/users/get.rs` and `api/v1/users/post.rs` and also the third file `api/v1/users/mod.rs` which will
//! contain a service with the scope for the `users` part of the path, that means that all endpoints under `api/v1/users/` will be relative
//! to `/api/v1/users` and should be treated as such.

use actix_oauth::dto::TokenResponse;
use actix_oauth::handler::Oauth2HandlerBuilder;
use actix_oauth::oauth;
use actix_oauth::types::{ClientId, ClientSecret, Password, RefreshToken, Username};
use actix_web::{web, HttpRequest};
use api::api;
use tracing::info;

pub(crate) mod api;
pub(crate) mod health;
mod not_found;
mod test;

pub(crate) use health::*;

#[oauth]
async fn password(_: HttpRequest, _username: Username, _password: Password) {
    info!("User tries to login");

    Ok(TokenResponse::new())
}

#[oauth]
async fn refresh(_: HttpRequest, _: Option<ClientId>, _: Option<ClientSecret>, _: RefreshToken) {
    Ok(TokenResponse::new())
}

pub(crate) fn index_scope() -> impl actix_web::dev::HttpServiceFactory {
    let oauth_handler = Oauth2HandlerBuilder::new()
        .password_handler(password)
        .refresh_handler(refresh)
        .build();

    web::scope("")
        .service(oauth_handler)
        .service(health::health)
        .service(api())
        .default_service(web::to(not_found::not_found))
}
