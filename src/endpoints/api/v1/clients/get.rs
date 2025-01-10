use crate::error::ApiError;
use crate::models::oauth_client::OAuthClient;
use crate::repositories::oauth_clients::OauthClientsRepository;
use crate::traits::into_dto::IntoDTO;
use crate::traits::repository::Repository;
use actix_helper_utils::generate_endpoint;
use actix_oauth::dto::OAuthClientDTOCollection;
use actix_web::web;

generate_endpoint! {
    /// Gets all registered OAuth clients
    fn get_clients;
    method: get;
    path: "";
    return_type: OAuthClientDTOCollection;
    error: ApiError;
    docs: {
        tag: "Client",
        context_path: "/clients",
    }
    params: {
        repository: web::Data<OauthClientsRepository>
    }
    {
        let clients: Vec<OAuthClient> = repository.get_all().await?;
        Ok(clients.into_dto())
    }
}
