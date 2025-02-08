use crate::models::oauth_token::{OAuthToken, TokenType};
use crate::repositories::oauth_token::OAUTH_TOKEN_REPOSITORY;
use crate::{ApiResult, ServerResult};
use actix_oauth::dto::TokenResponse;
use actix_oauth::handler::{OAuth2Handler, OAuth2HandlerBuilder};
use chrono::{Local, TimeDelta};
use sqlx_utils::traits::Repository;
use uuid::Uuid;

mod password_handler;

#[inline]
pub(crate) async fn oauth_handler() -> ServerResult<OAuth2Handler> {
    Ok(OAuth2HandlerBuilder::new()
        .password_handler(password_handler::password_handler)
        .build())
}

async fn create_token_response(user_ext_id: Uuid) -> ApiResult<TokenResponse> {
    let token = TokenResponse::new();
    let token_repo = *OAUTH_TOKEN_REPOSITORY;

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
