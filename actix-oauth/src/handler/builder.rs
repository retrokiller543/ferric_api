use super::OAuth2HandlerBuilder;
use crate::dto::AuthorizationRequest;
use crate::handler::{AuthorizationFuture, HandlerFuture};
use crate::types::{
    AuthorizationCode, ClientId, ClientSecret, Password, RedirectUri, RefreshToken, Username,
};
use actix_web::HttpRequest;
use std::sync::Arc;

impl OAuth2HandlerBuilder {
    pub fn password_handler(
        mut self,
        handler: impl Fn(HttpRequest, Username, Password) -> HandlerFuture + 'static,
    ) -> Self {
        self.password_grant_handler = Some(Arc::new(handler));
        self
    }

    pub fn authorization_code_handler(
        mut self,
        handler: impl Fn(HttpRequest, AuthorizationCode, RedirectUri, ClientId, ClientSecret) -> HandlerFuture
            + 'static,
    ) -> Self {
        self.authorization_code_grant_handler = Some(Arc::new(handler));
        self
    }

    pub fn authorization_handler(
        mut self,
        handler: impl Fn(HttpRequest, AuthorizationRequest) -> AuthorizationFuture + 'static,
    ) -> Self {
        self.authorization_handler = Some(Arc::new(handler));

        self
    }

    pub fn client_credentials_handler(
        mut self,
        handler: impl Fn(HttpRequest, ClientId, ClientSecret) -> HandlerFuture + 'static,
    ) -> Self {
        self.client_credentials_grant_handler = Some(Arc::new(handler));
        self
    }

    pub fn refresh_handler(
        mut self,
        handler: impl Fn(HttpRequest, Option<ClientId>, Option<ClientSecret>, RefreshToken) -> HandlerFuture
            + 'static,
    ) -> Self {
        self.refresh_token_handler = Some(Arc::new(handler));
        self
    }
}
