use crate::handler::HandlerReturn;
use crate::types::{ClientId, ClientSecret, RefreshToken};
use actix_web::HttpRequest;

pub trait RefreshTokenHandler:
    AsyncFn(HttpRequest, Option<ClientId>, Option<ClientSecret>, RefreshToken) -> HandlerReturn
    + Send
    + Sync
    + Clone
    + 'static
{
}

impl<T> RefreshTokenHandler for T where
    T: AsyncFn(HttpRequest, Option<ClientId>, Option<ClientSecret>, RefreshToken) -> HandlerReturn
        + Send
        + Sync
        + Clone
        + 'static
{
}
