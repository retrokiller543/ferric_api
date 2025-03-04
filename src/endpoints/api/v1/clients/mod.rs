use crate::utils::api_scope;

pub(crate) mod get;
pub(crate) mod post;

api_scope! {
    pub(super) clients = "/clients";

    paths: [get::get_clients, post::register];

    docs: {
        schemas: [actix_oauth::dto::OAuthCreateClientDTO, actix_oauth::dto::OAuthClientDTOCollection, actix_oauth::dto::OAuthClientDTO];
        responses: [actix_oauth::dto::OAuthClientDTOCollection, actix_oauth::dto::OAuthClientDTO];
    }
}
