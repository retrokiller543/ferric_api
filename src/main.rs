use crate::error::{ApiError, ServerError};
use crate::setup::server;
use crate::setup::setup;
use once_cell::sync::Lazy;
use tosic_utils::env::env_util;

pub(crate) mod config;
pub(crate) mod dto;
pub(crate) mod endpoints;
pub(crate) mod env;
pub(crate) mod error;
pub(crate) mod logging;
pub(crate) mod openapi;
pub(crate) mod services;
pub(crate) mod setup;
pub(crate) mod state;

pub(crate) type ApiResult<T> = Result<T, ApiError>;
pub(crate) type ServerResult<T> = Result<T, ServerError>;

static PORT: Lazy<u32> = Lazy::new(|| env_util!("PORT", 8000, u32));
static BASE_URL: Lazy<String> = Lazy::new(|| env_util!("BASE_URL", "http://localhost:8000"));

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
