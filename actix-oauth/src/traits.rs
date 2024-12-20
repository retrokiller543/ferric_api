#![allow(dead_code)]

//! This might be used in the future if I can find a way to make the state available each handler

use crate::handler::{HandlerFuture, HandlerReturn};
use crate::types::{Password, Username};
use std::future::Future;

pub trait OauthHandler<T>: 'static {
    fn call(&self, params: T) -> HandlerFuture;
}

impl<T, F> OauthHandler<(Username, Password)> for T
where
    T: Fn(Username, Password) -> F + 'static,
    F: Future<Output = HandlerReturn> + Send + 'static,
{
    fn call(&self, (username, password): (Username, Password)) -> HandlerFuture {
        Box::pin(self(username, password))
    }
}
