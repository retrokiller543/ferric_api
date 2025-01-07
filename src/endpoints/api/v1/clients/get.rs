use crate::dto::IntoDTO;
use crate::error::ApiError;
use crate::repositories::oauth_clients::OauthClientsRepository;
use crate::repositories::Repository;
use actix_helper_utils::generate_endpoint;
use actix_web::web;

generate_endpoint! {
    /// Gets all registered OAuth clients
    fn get_clients;
    method: get;
    path: "";
    error: ApiError;
    docs: {
        tag: "Client",
        context_path: "/clients",
    }
    params: {
        repository: web::Data<OauthClientsRepository>
    }
    {
        let clients = repository.get_all().await?;
        Ok(web::Json(clients.into_dto()))
    }
}
