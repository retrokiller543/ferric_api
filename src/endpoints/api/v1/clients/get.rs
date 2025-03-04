use crate::error::ApiError;
use crate::models::oauth_client::OAuthClient;
use crate::repositories::oauth_clients::OauthClientsRepository;
use crate::traits::into_dto::IntoDTO;
use actix_helper_utils::generate_endpoint;
use actix_oauth::dto::OAuthClientDTOCollection;
use actix_web::web;
use sqlx_utils::traits::Repository;

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
        responses: {
            (status = 200, response = OAuthClientDTOCollection)
        }
    }
    params: {
        repository: web::Data<OauthClientsRepository>
    }
    {
        let clients: Vec<OAuthClient> = repository.get_all().await?;
        Ok(clients.into_dto())
    }
}
