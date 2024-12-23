use crate::error::{ApiError, ServerError};
use crate::setup::server;
use crate::setup::setup;
use models::oauth_client::OAuthClient;
use once_cell::sync::Lazy;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::sync::LazyLock;
use tokio::sync::OnceCell;
use tosic_utils::env::env_util;

pub(crate) mod config;
pub(crate) mod dto;
pub(crate) mod endpoints;
pub(crate) mod env;
pub(crate) mod error;
pub(crate) mod logging;
mod models;
pub(crate) mod openapi;
pub(crate) mod repositories;
pub(crate) mod services;
pub(crate) mod setup;
pub(crate) mod state;
mod types;

pub(crate) type ApiResult<T> = Result<T, ApiError>;
pub(crate) type ServerResult<T> = Result<T, ServerError>;

pub(crate) static PORT: Lazy<u32> = Lazy::new(|| env_util!("PORT", 8000, u32));
pub(crate) static BASE_URL: Lazy<String> =
    Lazy::new(|| env_util!("BASE_URL", "http://localhost:8000"));
pub(crate) static DATABASE_URL: LazyLock<String> = LazyLock::new(|| env_util!("DATABASE_URL"));
pub(crate) static DB_POOL: OnceCell<PgPool> = OnceCell::const_new();

#[actix::main]
async fn main() -> ServerResult<()> {
    setup().await?;

    server!()
        .bind(format!("0.0.0.0:{}", *PORT))?
        .bind(format!("[::1]:{}", *PORT))?
        .run()
        .await?;

    Ok(())
}
