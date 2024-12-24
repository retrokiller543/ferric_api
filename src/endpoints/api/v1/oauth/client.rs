use crate::dto::IntoDTO;
use crate::error::ApiError;
use crate::models::oauth_client::OAuthClient;
use crate::repositories::oauth_clients::OAuthClientsRepository;
use crate::repositories::Repository;
use actix_helper_utils::generate_endpoint;
use actix_oauth::dto::create::OAuthCreateClientDTO;
use actix_web::web;
use validator::Validate;

generate_endpoint! {
    fn get_clients;
    method: get;
    path: "/clients";
    error: ApiError;
    params: {
        repository: web::Data<OAuthClientsRepository>
    }
    {
        let clients = repository.get_all().await?;
        Ok(web::Json(clients.into_dto()))
    }
}

generate_endpoint! {
    fn register;
    method: post;
    path: "/clients";
    error: ApiError;
    params: {
        repository: web::Data<OAuthClientsRepository>,
        web::Json(dto): web::Json<OAuthCreateClientDTO>
    };
    docs: {
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
