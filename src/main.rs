use crate::error::{ApiError, ServerError};
use crate::setup::server;
use crate::setup::setup;
use statics::PORT;

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
pub(crate) mod statics;
pub(crate) mod types;
pub(crate) mod utils;

pub(crate) type ApiResult<T> = Result<T, ApiError>;
pub(crate) type ServerResult<T> = Result<T, ServerError>;

#[actix::main]
async fn main() -> ServerResult<()> {
    let _guard = setup().await?;

    server!()
        .bind(format!("0.0.0.0:{}", *PORT))?
        .bind(format!("[::1]:{}", *PORT))?
        .run()
        .await?;

    Ok(())
}
