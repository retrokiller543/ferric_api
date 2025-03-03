use crate::handler::HandlerReturn;
use crate::types::{AuthorizationCode, ClientId, ClientSecret, RedirectUri};
use actix_web::HttpRequest;

pub trait AuthCodeHandler:
    AsyncFn(HttpRequest, AuthorizationCode, RedirectUri, ClientId, ClientSecret) -> HandlerReturn
    + Send
    + Sync
    + Clone
    + 'static
{
}

impl<T> AuthCodeHandler for T where
    T: AsyncFn(
            HttpRequest,
            AuthorizationCode,
            RedirectUri,
            ClientId,
            ClientSecret,
        ) -> HandlerReturn
        + Send
        + Sync
        + Clone
        + 'static
{
}
