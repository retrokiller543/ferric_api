//! All endpoints that this API serves, the structure of this module is built as closely as possible to the actual endpoints' path.
//!
//! # Example
//!
//! If we have the endpoint `GET /api/v1/users` then the file path will be, relative to the endpoints module, `api/v1/users`.
//! If we add another endpoint with the same path but different HTTP method, for example we also have a path `POST /api/v1/users`
//! then we have the file paths `api/v1/users/get.rs` and `api/v1/users/post.rs` and also the third file `api/v1/users/mod.rs` which will
//! contain a service with the scope for the `users` part of the path, that means that all endpoints under `api/v1/users/` will be relative
//! to `/api/v1/users` and should be treated as such.

use actix_web::web;
use api::api;

pub(crate) mod api;
pub(crate) mod health;
mod not_found;
mod test;

use crate::ServerResult;

#[inline]
pub fn index_scope() -> ServerResult<impl actix_web::dev::HttpServiceFactory> {
    let service = futures::executor::block_on(api())?;

    Ok(web::scope("")
        .service(health::health)
        .service(service)
        .default_service(web::to(not_found::not_found)))
}
