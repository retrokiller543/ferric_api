use crate::error::{ApiError, ServerError};
use crate::setup::server;
use crate::setup::setup;

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

#[actix::main]
async fn main() -> ServerResult<()> {
    setup().await?;

    let port = tosic_utils::env::env_util!("PORT", "8069");

    server!()
        .bind(format!("0.0.0.0:{port}"))?
        .bind(format!("[::1]:{port}"))?
        .run()
        .await?;

    Ok(())
}
