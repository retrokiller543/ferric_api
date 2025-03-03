use crate::ServerResult;
use crate::error::ServerError;
use dotenvy::{dotenv, from_filename};

/// Init the environment variables
///
/// If the `local` feature is enabled, it will load the `.env.local` file
///
/// If the `production` feature is enabled, it will load the `.env.production` file
///
/// If both features are enabled, it will load the `.env.local` file
#[inline]
pub fn init_env() -> ServerResult<()> {
    dotenv().ok();

    #[cfg(feature = "local")]
    match from_filename(".env.local") {
        Ok(_) => {}
        Err(err) => {
            return Err(ServerError::Basic(format!(
                "Error loading .env.local: {}",
                err
            )));
        }
    };

    #[cfg(all(feature = "production", not(feature = "local")))]
    match from_filename(".env.production") {
        Ok(_) => {}
        Err(err) => {
            return Err(ServerError::Basic(format!(
                "Error loading .env.production: {}",
                err
            )));
        }
    };

    Ok(())
}
