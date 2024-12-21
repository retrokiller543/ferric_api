use crate::dto::AuthorizationRequest;
use crate::handler::{AuthorizationFuture, HandlerField, HandlerFuture, Oauth2Handler};
use crate::types::{
    AuthorizationCode, ClientId, ClientSecret, Password, RedirectUri, RefreshToken, Username,
};
use actix_web::HttpRequest;
use std::sync::Arc;

pub struct Oauth2HandlerBuilder {
    password_grant_handler:
        Option<HandlerField<dyn Fn(HttpRequest, Username, Password) -> HandlerFuture>>,
    authorization_code_grant_handler: Option<
        HandlerField<
            dyn Fn(
                HttpRequest,
                AuthorizationCode,
                RedirectUri,
                ClientId,
                ClientSecret,
            ) -> HandlerFuture,
        >,
    >,
    client_credentials_grant_handler:
        Option<HandlerField<dyn Fn(HttpRequest, ClientId, ClientSecret) -> HandlerFuture>>,
    refresh_token_handler: Option<
        HandlerField<
            dyn Fn(
                HttpRequest,
                Option<ClientId>,
                Option<ClientSecret>,
                RefreshToken,
            ) -> HandlerFuture,
        >,
    >,

    authorization_handler:
        Option<HandlerField<dyn Fn(HttpRequest, AuthorizationRequest) -> AuthorizationFuture>>,
}

impl Oauth2HandlerBuilder {
    pub fn new() -> Self {
        Self {
            password_grant_handler: None,
            authorization_code_grant_handler: None,
            client_credentials_grant_handler: None,
            refresh_token_handler: None,
            authorization_handler: None,
        }
    }

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

    pub fn build(self) -> Oauth2Handler {
        Oauth2Handler {
            password_grant_handler: self.password_grant_handler,
            authorization_code_grant_handler: self.authorization_code_grant_handler,
            client_credentials_grant_handler: self.client_credentials_grant_handler,
            refresh_token_handler: self.refresh_token_handler,
            authorization_handler: self.authorization_handler,
        }
    }
}
