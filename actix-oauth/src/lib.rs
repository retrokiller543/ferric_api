#![feature(unboxed_closures)]
#![feature(async_fn_traits)]

use std::collections::BTreeMap;
use utoipa::{Modify, OpenApi};

pub mod dto;
pub mod error;
pub mod handler;
pub mod traits;
pub mod types;
mod utils;

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
            Scopes,
            ResponseType,
            OAuthClientDTO,
            OAuthCreateClientDTO,
            OAuthUpdateClientDTO
        ),
        responses(TokenResponse)
    ),
    tags(
        (name = "OAuth", description = "Oauth2 related endpoints"),
    ),
    modifiers(&NormalizePath)
)]
pub struct OauthAPI;
