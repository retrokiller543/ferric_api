//! Contains almost all configurations and setup needed to run the app, this includes macros to actually construct the
//! app and server so that we can more simply run the app in tests in the same way we run the app in production

pub mod app;
pub mod database;
pub mod server;

use crate::ServerResult;
use crate::env::init_env;
use crate::logging::{TracingGuard, init_tracing};

#[inline]
pub async fn setup() -> ServerResult<TracingGuard> {
    init_env()?;

    init_tracing()
}
