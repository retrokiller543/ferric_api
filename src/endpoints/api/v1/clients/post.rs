use crate::error::ApiError;
use crate::models::oauth_client::OAuthClient;
use crate::repositories::oauth_clients::OauthClientsRepository;
use actix_helper_utils::generate_endpoint;
use actix_oauth::dto::OAuthCreateClientDTO;
use actix_web::web;
use sqlx_utils::traits::Repository;
use validator::Validate;

generate_endpoint! {
    /// Register a new client
    fn register;
    method: post;
    path: "";
    error: ApiError;
    params: {
        repository: web::Data<OauthClientsRepository>,
        web::Json(dto): web::Json<OAuthCreateClientDTO>
    };
    docs: {
        tag: "Client",
        context_path: "/clients",
        request_body: {
            description = "Details needed to create a new OAuth Client",
            content(
                (OAuthCreateClientDTO)
            )
        }
    }
    {
        match dto.validate() {
            Ok(()) => {},
            Err(error) => return Err(error.into())
        }

        let model = OAuthClient::new(dto);
        repository.insert(&model).await?;
        Ok(web::Json(repository.get_by_id(model.client_id).await?))
    }
}
