use crate::dto::*;
use crate::endpoints::api::v1::ai::ai_service;
use crate::openapi::NormalizePath;
use crate::services::oauth::oauth_handler;
use crate::utils::api_scope;
use actix_oauth::OauthAPI;
use clients::clients_service;
use users::users_service;

mod ai;
pub mod clients;
mod users;

api_scope! {
    pub(crate) v1 = "/v1";

    version: V1;
    services: [clients_service, oauth_handler, users_service, ai_service];

    docs: {
        schemas: [Error];
        responses: [Error];
        nested: [
            ("/", clients::ClientsAPI),
            ("/", users::UsersAPI),
            ("/", ai::AiAPI),
            ("/", OauthAPI),
        ];
        modifiers: [NormalizePath];
    }
}
