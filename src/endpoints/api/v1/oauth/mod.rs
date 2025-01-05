use actix_web::dev::HttpServiceFactory;
use actix_web::web;

pub(crate) mod client;

#[inline]
pub(crate) fn oauth_inners() -> impl HttpServiceFactory {
    web::scope("")
        .service(client::get_clients)
        .service(client::register)
}
