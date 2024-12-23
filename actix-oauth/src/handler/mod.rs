pub mod builder;
pub use builder::*;
pub(crate) mod docs;

use crate::dto::token_response::TokenResponse;
use crate::dto::{AuthorizationRequest, OauthRequest};
use crate::error::Oauth2ErrorType;
use crate::types::{
    AuthorizationCode, ClientId, ClientSecret, Password, RedirectUri, RefreshToken, Username,
};
use actix_web::dev::{AppService, HttpServiceFactory};
use actix_web::web::post;
use actix_web::{web, HttpRequest, HttpResponse};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tracing::instrument;

pub type OAuthFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;
pub type HandlerReturn = Result<TokenResponse, Oauth2ErrorType>;
pub type AuthorizationReturn = Result<HttpResponse, Oauth2ErrorType>;
pub type HandlerFuture = OAuthFuture<HandlerReturn>;
pub type AuthorizationFuture = OAuthFuture<AuthorizationReturn>;

pub(crate) type HandlerField<H> = Arc<H>;

#[derive(Clone)]
pub struct Oauth2Handler {
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

impl Oauth2Handler {
    #[instrument(skip(self), level = "debug")]
    async fn token_handler(&self, req: HttpRequest, oauth_req: OauthRequest) -> HandlerReturn {
        let password_handler = self.password_grant_handler.clone();
        let authorization_code_handler = self.authorization_code_grant_handler.clone();
        let client_credentials_handler = self.client_credentials_grant_handler.clone();
        let refresh_handler = self.refresh_token_handler.clone();

        match oauth_req {
            OauthRequest::Password { username, password } => {
                if let Some(method) = password_handler {
                    method(req, username, password).await
                } else {
                    Err(Oauth2ErrorType::UnsupportedGrantType)
                }
            }
            OauthRequest::AuthorizationCode {
                code,
                redirect_uri,
                client_id,
                client_secret,
            } => {
                if let Some(method) = authorization_code_handler {
                    method(req, code, redirect_uri, client_id, client_secret).await
                } else {
                    Err(Oauth2ErrorType::UnsupportedGrantType)
                }
            }
            OauthRequest::ClientCredentials {
                client_id,
                client_secret,
            } => {
                if let Some(method) = client_credentials_handler {
                    method(req, client_id, client_secret).await
                } else {
                    Err(Oauth2ErrorType::UnsupportedGrantType)
                }
            }
            OauthRequest::RefreshToken {
                client_id,
                client_secret,
                refresh_token,
            } => {
                if let Some(method) = refresh_handler {
                    method(req, client_id, client_secret, refresh_token).await
                } else {
                    Err(Oauth2ErrorType::UnsupportedGrantType)
                }
            }
        }
    }
}

impl HttpServiceFactory for Oauth2Handler {
    fn register(self, config: &mut AppService) {
        let handler = Arc::new(self);

        let token_handler = {
            let handler = Arc::clone(&handler);

            move |req: HttpRequest,
                  oauth_req: web::Either<
                web::Either<web::Form<OauthRequest>, web::Json<OauthRequest>>,
                web::Query<OauthRequest>,
            >| {
                let oauth_req = match oauth_req {
                    web::Either::Left(web::Either::Left(web::Form(oauth_request))) => oauth_request,
                    web::Either::Left(web::Either::Right(web::Json(oauth_request))) => {
                        oauth_request
                    }
                    web::Either::Right(web::Query(oauth_request)) => oauth_request,
                };
                let handler = Arc::clone(&handler);
                async move { handler.token_handler(req, oauth_req).await }
            }
        };

        let authorization_handler_inner = handler.authorization_handler.clone();
        let authorization_handler =
            move |req: HttpRequest,
                  web::Query(authorization_request): web::Query<AuthorizationRequest>| {
                let authorization_handler_inner = authorization_handler_inner.clone();

                async move {
                    if let Some(handler) = authorization_handler_inner {
                        handler(req, authorization_request).await
                    } else {
                        Err(Oauth2ErrorType::UnsupportedGrantType)
                    }
                }
            };

        let scope = web::scope("/oauth")
            .route("/token", post().to(token_handler))
            .route("/authorize", post().to(authorization_handler));

        HttpServiceFactory::register(scope, config);
    }
}
