use crate::handler::HandlerReturn;
use crate::types::{ClientId, ClientSecret};
use actix_web::HttpRequest;

pub trait ClientCredentialsHandler:
    AsyncFn(HttpRequest, ClientId, ClientSecret) -> HandlerReturn + Send + Sync + Clone + 'static
{
}

impl<T> ClientCredentialsHandler for T where
    T: AsyncFn(HttpRequest, ClientId, ClientSecret) -> HandlerReturn
        + Send
        + Sync
        + Clone
        + 'static
{
}
