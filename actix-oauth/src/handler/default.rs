use crate::dto::AuthorizationRequest;
use crate::error::Oauth2ErrorType;
use crate::handler::{AuthorizationReturn, HandlerReturn};
use crate::oauth2_handler;
use crate::types::{
    AuthorizationCode, ClientId, ClientSecret, Password, RedirectUri, RefreshToken, Username,
};
use actix_web::HttpRequest;

oauth2_handler! {
    pub fn NotImplementedPasswordHandler(_ => (HttpRequest, Username, Password)) -> HandlerReturn
}

oauth2_handler! {
    pub fn NotImplementedAuthCodeHandler(_ => (HttpRequest, AuthorizationCode, RedirectUri, ClientId, ClientSecret)) -> HandlerReturn
}

oauth2_handler! {
    pub fn NotImplementedClientCredentialsHandler(_ => (HttpRequest, ClientId, ClientSecret)) -> HandlerReturn
}

oauth2_handler! {
    pub fn NotImplementedRefreshTokenHandler(_ => (HttpRequest, Option<ClientId>, Option<ClientSecret>, RefreshToken)) -> HandlerReturn
}

oauth2_handler! {
    pub fn NotImplementedAuthorizationHandler(_ => (HttpRequest, AuthorizationRequest)) -> AuthorizationReturn
}
