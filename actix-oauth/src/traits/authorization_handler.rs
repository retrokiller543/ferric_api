use crate::dto::AuthorizationRequest;
use crate::handler::AuthorizationReturn;
use actix_web::HttpRequest;

pub trait AuthorizationHandler:
    AsyncFn(HttpRequest, AuthorizationRequest) -> AuthorizationReturn + Send + Sync + Clone + 'static
{
}

impl<T> AuthorizationHandler for T where
    T: AsyncFn(HttpRequest, AuthorizationRequest) -> AuthorizationReturn
        + Send
        + Sync
        + Clone
        + 'static
{
}
