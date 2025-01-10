use crate::models::oauth_token::{OAuthToken, TokenType};
use crate::repositories::oauth_token::get_oauth_token_repository;
use crate::traits::repository::Repository;
use crate::ApiResult;
use actix_oauth::dto::TokenResponse;
use actix_oauth::handler::{OAuth2Handler, OAuth2HandlerBuilder};
use chrono::{Local, TimeDelta};
use uuid::Uuid;

mod password_handler;

#[inline]
pub(crate) fn oauth_handler() -> OAuth2Handler {
    OAuth2HandlerBuilder::new()
        .password_handler(password_handler::password_handler)
        .build()
}

async fn create_token_response(user_ext_id: Uuid) -> ApiResult<TokenResponse> {
    let token = TokenResponse::new();
    let token_repo = get_oauth_token_repository().await?;

    let expires = Local::now()
        .checked_add_signed(TimeDelta::seconds(token.expires_in as i64))
        .unwrap()
        .naive_utc();

    let access_token = OAuthToken::new(
        token.access_token.secret().to_string(),
        user_ext_id,
        TokenType::Access,
        expires,
    );
    let refresh_token = OAuthToken::new(
        token.refresh_token.secret().to_string(),
        user_ext_id,
        TokenType::Refresh,
        expires,
    );

    token_repo.save_all([access_token, refresh_token]).await?;

    Ok(token)
}
