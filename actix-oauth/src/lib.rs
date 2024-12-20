use std::collections::BTreeMap;
use utoipa::{Modify, OpenApi};

pub mod dto;
pub mod error;
pub mod handler;
mod traits;
pub mod types;

pub use actix_oauth_macro::oauth;
use dto::{Oauth2Error, OauthRequest, TokenResponse, TokenType};
use types::{
    AccessToken, AuthorizationCode, ClientId, ClientSecret, Password, RefreshToken, Username,
};
use utoipa::openapi::OpenApi as OpenApiSpec;

struct NormalizePath;

impl Modify for NormalizePath {
    fn modify(&self, openapi: &mut OpenApiSpec) {
        let paths = openapi.paths.paths.clone();
        let mut new_paths = BTreeMap::new();

        paths.iter().for_each(|(path, item)| {
            let new_path = &path.replace("//", "/");

            new_paths.insert(new_path.clone(), item.clone());
        });

        openapi.paths.paths = new_paths;
    }
}

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
    ),
    tags(
        (name = "Oauth", description = "Oauth2 related endpoints")
    ),
    modifiers(&NormalizePath)
)]
pub struct OauthAPI;
