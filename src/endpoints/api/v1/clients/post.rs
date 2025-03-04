use crate::dto::Error;
use crate::error::ApiError;
use crate::models::oauth_client::OAuthClient;
use crate::repositories::oauth_clients::OauthClientsRepository;
use crate::traits::IntoDTO;
use actix_helper_utils::generate_endpoint;
use actix_oauth::dto::{OAuthClientDTO, OAuthCreateClientDTO};
use actix_web::web;
use sqlx_utils::traits::Repository;
use tracing::error;
use validator::Validate;

generate_endpoint! {
    /// Register a new oauth2 client.
    ///
    /// # Returns
    ///
    /// * `OAuthClientDTO` - The created client.
    fn register;
    method: post;
    path: "";
    error: ApiError;
    return_type: OAuthClientDTO
    docs: {
        tag: "Client",
        context_path: "/clients",
        request_body: {
            description = "Details needed to create a new OAuth Client",
            content(
                (OAuthCreateClientDTO)
            )
        }
        responses: {
            (status = 200, description = "Successfully created a new OAuth client", body = OAuthClientDTO),
            (status = 500, description = "Internal Server Error", body = Error)
        }
    }
    params: {
        repository: web::Data<OauthClientsRepository>,
        web::Json(dto): web::Json<OAuthCreateClientDTO>
    };
    {
        match dto.validate() {
            Ok(()) => {},
            Err(error) => return Err(error.into())
        }

        let model = OAuthClient::new(dto);
        repository.insert(&model).await?;
        let client = repository.get_by_id(model.client_id).await?;

        match client {
            Some(client) => Ok(client.into_dto()),
            None => {
                error!("Unknown issue while creating OAuth client");
                Err(ApiError::InternalError)
            }
        }
    }
}
