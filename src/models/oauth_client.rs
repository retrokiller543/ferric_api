use crate::traits::into_dto::IntoDTO;
use crate::traits::model::Model;
use crate::traits::FromModel;
use actix_oauth::dto::create::OAuthCreateClientDTO;
use actix_oauth::dto::{OAuthClientDTO, OAuthClientDTOCollection};
use actix_oauth::types::{ClientId, ClientSecret, GrantType, RedirectUri, Scopes};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(
    Default, Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, FromRow, Serialize, Deserialize,
)]
pub(crate) struct OAuthClient {
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
    pub(crate) redirect_uri: String,
    pub(crate) grant_types: Vec<GrantType>,
    pub(crate) scopes: Vec<String>,
    pub(crate) created_at: Option<NaiveDateTime>,
}

impl OAuthClient {
    pub fn new(dto: OAuthCreateClientDTO) -> Self {
        let id = ClientId::new_random();
        let secret = ClientSecret::new_random();

        Self {
            client_id: id.to_string(),
            client_secret: secret.secret().to_string(),
            redirect_uri: dto.redirect_uri.to_string(),
            grant_types: dto.grant_types,
            scopes: dto.scopes.to_vec(),
            created_at: None,
        }
    }
}

/*impl IntoDTO<OAuthClientDTO> for OAuthClient {
    fn into_dto(self) -> OAuthClientDTO {
        OAuthClientDTO {
            client_id: ClientId::new(self.client_id),
            client_secret: ClientSecret::new(self.client_secret),
            redirect_uri: RedirectUri::new(self.redirect_uri),
            grant_types: self.grant_types,
            scopes: Scopes::from_iter(self.scopes),
            created_at: self
                .created_at
                .expect("Expected 'created_at' to be populated"),
        }
    }
}*/

impl FromModel<OAuthClient> for OAuthClientDTO {
    fn from_model(model: OAuthClient) -> Self {
        Self {
            client_id: ClientId::new(model.client_id),
            client_secret: ClientSecret::new(model.client_secret),
            redirect_uri: RedirectUri::new(model.redirect_uri),
            grant_types: model.grant_types,
            scopes: Scopes::from_iter(model.scopes),
            created_at: model
                .created_at
                .expect("Expected 'created_at' to be populated"),
        }
    }
}

impl FromModel<Vec<OAuthClient>> for OAuthClientDTOCollection {
    fn from_model(model: Vec<OAuthClient>) -> Self {
        let dto: Vec<OAuthClientDTO> = model.into_dto();
        dto.into()
    }
}

impl Model for OAuthClient {
    type Id = String;

    fn get_id(&self) -> Option<Self::Id> {
        Some(self.client_id.to_string())
    }
}
