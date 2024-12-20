use utoipa::OpenApi;

pub mod dto;
pub mod error;
pub mod handler;
pub mod types;
use dto::{Oauth2Error, OauthRequest, TokenResponse, TokenType};
use types::{
    AccessToken, AuthorizationCode, ClientId, ClientSecret, Password, RefreshToken, Username,
};
#[derive(OpenApi)]
#[openapi(
    paths(handler::Oauth2Handler),
    components(
        schemas(
            RefreshToken,
            AccessToken,
            AuthorizationCode,
            ClientSecret,
            ClientId,
            Password,
            Username,
            OauthRequest,
            TokenResponse,
            Oauth2Error,
            TokenType
        ),
        responses()
    )
)]
pub struct OauthAPI;
