use crate::handler::HandlerReturn;
use crate::types::{Password, Username};
use actix_web::HttpRequest;

pub trait PasswordHandler:
    AsyncFn(HttpRequest, Username, Password) -> HandlerReturn + Send + Sync + Clone + 'static
{
}

impl<T> PasswordHandler for T where
    T: AsyncFn(HttpRequest, Username, Password) -> HandlerReturn + Send + Sync + Clone + 'static
{
}
