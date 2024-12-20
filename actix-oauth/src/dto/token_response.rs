use crate::types::{AccessToken, RefreshToken};
use actix_web::body::BoxBody;
use actix_web::{HttpRequest, HttpResponse, Responder};
use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema, ToResponse)]
pub struct TokenResponse {
    pub(crate) access_token: AccessToken,
    pub(crate) refresh_token: RefreshToken,
    token_type: TokenType,
    expires_in: usize,
}

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

/// Returns a random alphanumeric string of length `length`.
fn random_string(length: usize) -> String {
    Alphanumeric.sample_string(&mut thread_rng(), length)
}

impl TokenResponse {
    pub fn new() -> Self {
        Self {
            access_token: AccessToken::new(random_string(50)),
            refresh_token: RefreshToken::new(random_string(50)),
            token_type: TokenType::default(),
            expires_in: 3600,
        }
    }
}

impl Responder for TokenResponse {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}
