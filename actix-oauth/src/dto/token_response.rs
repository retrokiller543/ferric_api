use crate::types::{AccessToken, RefreshToken};
use crate::{impl_responder, utils};
use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema, ToResponse)]
pub struct TokenResponse {
    pub access_token: AccessToken,
    pub refresh_token: RefreshToken,
    pub token_type: TokenType,
    pub expires_in: usize,
}

impl_responder!(TokenResponse);

#[derive(Debug, Serialize, Deserialize, ToSchema, Default, ToResponse)]
#[serde(rename_all = "lowercase")]
pub enum TokenType {
    #[default]
    Bearer,
    Basic,
}

#[derive(ToResponse)]
#[allow(dead_code)]
pub enum TokenResponseExample {
    #[response(examples(
            ("password" = (value = json!({
                "access_token": "ZzwuN7HvEw80MedCDOcQVRrnm3lhHBkmkpYK7TY1yDY7enjjmc",
                "refresh_token": "SMZuiT5rjv9UmfIXcYMvJQSHRRt8I8Dtg6U6o6C6SNCs80pE4o",
                "token_type": "bearer",
                "expires_in": 3600
            }), description = "Successful access token request and the credentials are returned", summary = "Successful access token request"))
    ))]
    Success(#[content("application/json")] TokenResponse),
}

impl Default for TokenResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenResponse {
    pub fn new() -> Self {
        Self {
            access_token: AccessToken::new(utils::random_string(50)),
            refresh_token: RefreshToken::new(utils::random_string(50)),
            token_type: TokenType::default(),
            expires_in: 3600,
        }
    }
}
