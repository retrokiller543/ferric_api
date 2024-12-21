use std::collections::BTreeMap;
use utoipa::{Modify, OpenApi};

pub mod dto;
pub mod error;
pub mod handler;
mod traits;
pub mod types;

pub use actix_oauth_macro::oauth;
use dto::*;
use types::*;
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
    paths(handler::docs::token, handler::docs::authorize),
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
            TokenType,
            RedirectUri,
            Scope,
            Scopes,
            AuthorizationRequest,
            ResponseType
        ),
        responses(TokenResponse)
    ),
    tags(
        (name = "OAuth", description = "Oauth2 related endpoints"),
        (name = "Auth", description = "Authentication related endpoints")
    ),
    modifiers(&NormalizePath)
)]
pub struct OauthAPI;
