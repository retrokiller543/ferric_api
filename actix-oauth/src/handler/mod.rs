mod builder;

mod default;
pub(crate) mod docs;
pub use builder::OAuth2HandlerBuilder;
use default::*;

use crate::dto::OauthRequest;
use crate::dto::token_response::TokenResponse;
use crate::error::Oauth2ErrorType;
use crate::traits::*;
use actix_web::dev::{AppService, HttpServiceFactory};
use actix_web::{HttpRequest, HttpResponse};

pub type HandlerReturn = Result<TokenResponse, Oauth2ErrorType>;
pub type AuthorizationReturn = Result<HttpResponse, Oauth2ErrorType>;

#[derive(Clone)]
pub struct OAuth2Handler<
    PH = NotImplementedPasswordHandler,
    AH = NotImplementedAuthCodeHandler,
    CH = NotImplementedClientCredentialsHandler,
    RH = NotImplementedRefreshTokenHandler,
    AuthH = NotImplementedAuthorizationHandler,
> where
    PH: PasswordHandler,
    AH: AuthCodeHandler,
    CH: ClientCredentialsHandler,
    RH: RefreshTokenHandler,
    AuthH: AuthorizationHandler,
{
    password_grant_handler: PH,
    authorization_code_grant_handler: AH,
    client_credentials_grant_handler: CH,
    refresh_token_handler: RH,
    authorization_handler: AuthH,
}

impl Default for OAuth2Handler {
    fn default() -> Self {
        OAuth2HandlerBuilder::new().build()
    }
}

impl<PH, AH, CH, RH, AuthH> OAuth2Manager for OAuth2Handler<PH, AH, CH, RH, AuthH>
where
    PH: PasswordHandler,
    AH: AuthCodeHandler,
    CH: ClientCredentialsHandler,
    RH: RefreshTokenHandler,
    AuthH: AuthorizationHandler,
{
    async fn token_handler(&self, req: HttpRequest, oauth_req: OauthRequest) -> HandlerReturn {
        match oauth_req {
            OauthRequest::Password { username, password } => {
                self.password_grant_handler
                    .async_call((req, username, password))
                    .await
            }
            OauthRequest::AuthorizationCode {
                code,
                redirect_uri,
                client_id,
                client_secret,
            } => {
                self.authorization_code_grant_handler
                    .async_call((req, code, redirect_uri, client_id, client_secret))
                    .await
            }
            OauthRequest::ClientCredentials {
                client_id,
                client_secret,
            } => {
                self.client_credentials_grant_handler
                    .async_call((req, client_id, client_secret))
                    .await
            }
            OauthRequest::RefreshToken {
                client_id,
                client_secret,
                refresh_token,
            } => {
                self.refresh_token_handler
                    .async_call((req, client_id, client_secret, refresh_token))
                    .await
            }
        }
    }

    fn authorization_handler(&self) -> impl AuthorizationHandler {
        self.authorization_handler.clone()
    }
}

impl<PH, AH, CH, RH, AuthH> HttpServiceFactory for OAuth2Handler<PH, AH, CH, RH, AuthH>
where
    PH: PasswordHandler,
    AH: AuthCodeHandler,
    CH: ClientCredentialsHandler,
    RH: RefreshTokenHandler,
    AuthH: AuthorizationHandler,
{
    fn register(self, config: &mut AppService) {
        self.into_service().register(config)
    }
}
