use crate::dto::AuthorizationRequest;
use crate::error::Oauth2ErrorType;
use crate::handler::{AuthorizationReturn, HandlerReturn};
use crate::types::{
    AuthorizationCode, ClientId, ClientSecret, Password, RedirectUri, RefreshToken, Username,
};
use actix_web::HttpRequest;

#[derive(Clone)]
pub struct NotImplementedPasswordHandler;
impl AsyncFnOnce<(HttpRequest, Username, Password)> for NotImplementedPasswordHandler {
    type CallOnceFuture = std::future::Ready<HandlerReturn>;
    type Output = HandlerReturn;

    extern "rust-call" fn async_call_once(
        self,
        _: (HttpRequest, Username, Password),
    ) -> Self::CallOnceFuture {
        std::future::ready(Err(Oauth2ErrorType::UnsupportedGrantType))
    }
}

impl AsyncFnMut<(HttpRequest, Username, Password)> for NotImplementedPasswordHandler {
    type CallRefFuture<'a>
        = std::future::Ready<HandlerReturn>
    where
        Self: 'a;

    extern "rust-call" fn async_call_mut(
        &mut self,
        _: (HttpRequest, Username, Password),
    ) -> Self::CallRefFuture<'_> {
        std::future::ready(Err(Oauth2ErrorType::UnsupportedGrantType))
    }
}

impl AsyncFn<(HttpRequest, Username, Password)> for NotImplementedPasswordHandler {
    extern "rust-call" fn async_call(
        &self,
        _: (HttpRequest, Username, Password),
    ) -> Self::CallRefFuture<'_> {
        std::future::ready(Err(Oauth2ErrorType::UnsupportedGrantType))
    }
}

#[derive(Clone)]
pub struct NotImplementedAuthCodeHandler;
impl
    AsyncFnOnce<(
        HttpRequest,
        AuthorizationCode,
        RedirectUri,
        ClientId,
        ClientSecret,
    )> for NotImplementedAuthCodeHandler
{
    type CallOnceFuture = std::future::Ready<HandlerReturn>;
    type Output = HandlerReturn;

    extern "rust-call" fn async_call_once(
        self,
        _: (
            HttpRequest,
            AuthorizationCode,
            RedirectUri,
            ClientId,
            ClientSecret,
        ),
    ) -> Self::CallOnceFuture {
        std::future::ready(Err(Oauth2ErrorType::UnsupportedGrantType))
    }
}

impl
    AsyncFnMut<(
        HttpRequest,
        AuthorizationCode,
        RedirectUri,
        ClientId,
        ClientSecret,
    )> for NotImplementedAuthCodeHandler
{
    type CallRefFuture<'a>
        = std::future::Ready<HandlerReturn>
    where
        Self: 'a;

    extern "rust-call" fn async_call_mut(
        &mut self,
        _: (
            HttpRequest,
            AuthorizationCode,
            RedirectUri,
            ClientId,
            ClientSecret,
        ),
    ) -> Self::CallRefFuture<'_> {
        std::future::ready(Err(Oauth2ErrorType::UnsupportedGrantType))
    }
}

impl
    AsyncFn<(
        HttpRequest,
        AuthorizationCode,
        RedirectUri,
        ClientId,
        ClientSecret,
    )> for NotImplementedAuthCodeHandler
{
    extern "rust-call" fn async_call(
        &self,
        _: (
            HttpRequest,
            AuthorizationCode,
            RedirectUri,
            ClientId,
            ClientSecret,
        ),
    ) -> Self::CallRefFuture<'_> {
        std::future::ready(Err(Oauth2ErrorType::UnsupportedGrantType))
    }
}

#[derive(Clone)]
pub struct NotImplementedClientCredentialsHandler;
impl AsyncFnOnce<(HttpRequest, ClientId, ClientSecret)> for NotImplementedClientCredentialsHandler {
    type CallOnceFuture = std::future::Ready<HandlerReturn>;
    type Output = HandlerReturn;

    extern "rust-call" fn async_call_once(
        self,
        _: (HttpRequest, ClientId, ClientSecret),
    ) -> Self::CallOnceFuture {
        std::future::ready(Err(Oauth2ErrorType::UnsupportedGrantType))
    }
}

impl AsyncFnMut<(HttpRequest, ClientId, ClientSecret)> for NotImplementedClientCredentialsHandler {
    type CallRefFuture<'a>
        = std::future::Ready<HandlerReturn>
    where
        Self: 'a;

    extern "rust-call" fn async_call_mut(
        &mut self,
        _: (HttpRequest, ClientId, ClientSecret),
    ) -> Self::CallRefFuture<'_> {
        std::future::ready(Err(Oauth2ErrorType::UnsupportedGrantType))
    }
}

impl AsyncFn<(HttpRequest, ClientId, ClientSecret)> for NotImplementedClientCredentialsHandler {
    extern "rust-call" fn async_call(
        &self,
        _: (HttpRequest, ClientId, ClientSecret),
    ) -> Self::CallRefFuture<'_> {
        std::future::ready(Err(Oauth2ErrorType::UnsupportedGrantType))
    }
}

#[derive(Clone)]
pub struct NotImplementedRefreshTokenHandler;
impl
    AsyncFnOnce<(
        HttpRequest,
        Option<ClientId>,
        Option<ClientSecret>,
        RefreshToken,
    )> for NotImplementedRefreshTokenHandler
{
    type CallOnceFuture = std::future::Ready<HandlerReturn>;
    type Output = HandlerReturn;

    extern "rust-call" fn async_call_once(
        self,
        _: (
            HttpRequest,
            Option<ClientId>,
            Option<ClientSecret>,
            RefreshToken,
        ),
    ) -> Self::CallOnceFuture {
        std::future::ready(Err(Oauth2ErrorType::UnsupportedGrantType))
    }
}

impl
    AsyncFnMut<(
        HttpRequest,
        Option<ClientId>,
        Option<ClientSecret>,
        RefreshToken,
    )> for NotImplementedRefreshTokenHandler
{
    type CallRefFuture<'a>
        = std::future::Ready<HandlerReturn>
    where
        Self: 'a;

    extern "rust-call" fn async_call_mut(
        &mut self,
        _: (
            HttpRequest,
            Option<ClientId>,
            Option<ClientSecret>,
            RefreshToken,
        ),
    ) -> Self::CallRefFuture<'_> {
        std::future::ready(Err(Oauth2ErrorType::UnsupportedGrantType))
    }
}

impl
    AsyncFn<(
        HttpRequest,
        Option<ClientId>,
        Option<ClientSecret>,
        RefreshToken,
    )> for NotImplementedRefreshTokenHandler
{
    extern "rust-call" fn async_call(
        &self,
        _: (
            HttpRequest,
            Option<ClientId>,
            Option<ClientSecret>,
            RefreshToken,
        ),
    ) -> Self::CallRefFuture<'_> {
        std::future::ready(Err(Oauth2ErrorType::UnsupportedGrantType))
    }
}

#[derive(Clone)]
pub struct NotImplementedAuthorizationHandler;
impl AsyncFnOnce<(HttpRequest, AuthorizationRequest)> for NotImplementedAuthorizationHandler {
    type CallOnceFuture = std::future::Ready<AuthorizationReturn>;
    type Output = AuthorizationReturn;

    extern "rust-call" fn async_call_once(
        self,
        _: (HttpRequest, AuthorizationRequest),
    ) -> Self::CallOnceFuture {
        std::future::ready(Err(Oauth2ErrorType::UnsupportedGrantType))
    }
}

impl AsyncFnMut<(HttpRequest, AuthorizationRequest)> for NotImplementedAuthorizationHandler {
    type CallRefFuture<'a>
        = std::future::Ready<AuthorizationReturn>
    where
        Self: 'a;

    extern "rust-call" fn async_call_mut(
        &mut self,
        _: (HttpRequest, AuthorizationRequest),
    ) -> Self::CallRefFuture<'_> {
        std::future::ready(Err(Oauth2ErrorType::UnsupportedGrantType))
    }
}

impl AsyncFn<(HttpRequest, AuthorizationRequest)> for NotImplementedAuthorizationHandler {
    extern "rust-call" fn async_call(
        &self,
        _: (HttpRequest, AuthorizationRequest),
    ) -> Self::CallRefFuture<'_> {
        std::future::ready(Err(Oauth2ErrorType::UnsupportedGrantType))
    }
}
