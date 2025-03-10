use crate::models::oauth_token::{OAuthToken, TokenType};
use crate::prelude::UserContext;
use crate::repositories::oauth_token::OAUTH_TOKEN_REPOSITORY;
use crate::{ApiResult, ServerResult};
use actix_oauth::dto::TokenResponse;
use actix_oauth::handler::OAuth2HandlerBuilder;
use actix_oauth::traits::OAuth2Manager;
use actix_web::dev::HttpServiceFactory;
use chrono::{Local, TimeDelta};
//use sqlx_utils::prelude::*;
use tokio::try_join;
use uuid::Uuid;

mod password_handler;

#[inline]
pub(crate) async fn oauth_handler() -> ServerResult<impl OAuth2Manager + HttpServiceFactory> {
    Ok(OAuth2HandlerBuilder::new()
        .password_handler(password_handler::password_handler)
        .build())
}

async fn create_token_response(user_ext_id: Uuid) -> ApiResult<TokenResponse> {
    let token = TokenResponse::new();
    let token_repo = *OAUTH_TOKEN_REPOSITORY;
    let context = UserContext::System;

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
    let fut1 = token_repo.save_with_context(&access_token, &context);

    let refresh_token = OAuthToken::new(
        token.refresh_token.secret().to_string(),
        user_ext_id,
        TokenType::Refresh,
        expires,
    );
    let fut2 = token_repo.save_with_context(&refresh_token, &context);

    try_join!(fut1, fut2)?;

    Ok(token)
}
