#![feature(associated_type_defaults)]
#![feature(tuple_trait)]
#![allow(async_fn_in_trait)]

use crate::error::{ApiError, ServerError};

pub mod config;
mod constants;
pub mod dto;
pub mod endpoints;
pub mod env;
pub mod error;
mod extractors;
pub mod logging;
pub mod middleware;
pub mod models;
pub mod openapi;
pub mod prelude;
pub mod repositories;
pub mod services;
pub mod setup;
pub mod state;
pub mod statics;
pub mod traits;
pub mod types;
pub mod utils;

pub type ApiResult<T> = Result<T, ApiError>;
pub type ServerResult<T> = Result<T, ServerError>;
