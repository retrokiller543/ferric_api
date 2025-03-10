use crate::ServerResult;
use crate::error::ServerError;
use cfg_if::cfg_if;
use dotenvy::{dotenv, from_filename};
use std::path::Path;

/// Attempts to load environment variables from the specified file path.
///
/// # Arguments
///
/// * `path` - A path to the environment file (e.g., ".env.local")
///
/// # Returns
///
/// * `ServerResult<()>` - Ok(()) if successful, or ServerError if loading fails
///
/// # Errors
///
/// Returns a `ServerError::Basic` with the original error message if the file cannot be loaded.
fn load_env_from_file(path: impl AsRef<Path>) -> ServerResult<()> {
    match from_filename(path) {
        Ok(_) => Ok(()),
        Err(err) => Err(ServerError::Basic(format!(
            "Error loading env file: {}",
            err
        ))),
    }
}

/// Initializes environment variables based on compilation features, build profiles, or custom profiles.
///
/// # Environment File Selection Logic
///
/// The function selects which environment file to load using the following priority rules:
///
/// 1. Custom profile specified via `RUST_ENV_PROFILE`:
///    - If this compile-time environment variable is set, loads `.env.{profile}`
///    - Example: `RUST_ENV_PROFILE=staging` would load `.env.staging`
///
/// 2. Feature flags set at compile time (if no custom profile):
///    - If the "local" feature is enabled: Uses `.env.local`
///    - Else if "production" feature is enabled (and "local" is not): Uses `.env.production`
///    - Else if build is a release build (not debug_assertions): Uses `.env.production`
///    - Otherwise (debug build with no special features): Uses `.env.local`
///
/// 3. File path overrides:
///    - `RUST_ENV_FILE`: Directly specifies an env file path (highest priority)
///    - `RUST_DEV_ENV_FILE`: Overrides the development environment file path (default: ".env.local")
///    - `RUST_PROD_ENV_FILE`: Overrides the production environment file path (default: ".env.production")
///
/// # Usage
///
/// ```
/// // Basic usage:
/// use ferric_api::env::init_env;
///
/// init_env()?;
///
/// // When compiling with features:
/// // cargo build --features="local"      // Will use .env.local
/// // cargo build --features="production" // Will use .env.production
/// // cargo build --release               // Will use .env.production (if no features specified)
///
/// // With custom profile:
/// // RUST_ENV_PROFILE=staging cargo build  // Will use .env.staging
/// // RUST_ENV_FILE=.my-special.env cargo build  // Will use .my-special.env regardless of profile
/// ```
///
/// # Note
///
/// This function first calls `dotenv()` to load the default `.env` file before
/// loading the environment-specific file according to the rules above.
///
/// # Returns
///
/// * `ServerResult<()>` - Ok(()) on success, or an error if environment loading fails
#[inline]
pub fn init_env() -> ServerResult<()> {
    // First load the default .env file
    dotenv()?;

    // Check if there's a direct file path override
    if let Some(env_file) = option_env!("RUST_ENV_FILE") {
        return load_env_from_file(env_file);
    }

    // Check if we have a custom profile
    if let Some(profile) = option_env!("RUST_ENV_PROFILE") {
        let profile_env_file = format!(".env.{}", profile);
        return load_env_from_file(profile_env_file);
    }

    // Fall back to feature/build profile based selection
    cfg_if! {
        if #[cfg(feature = "local")] {
            // "local" feature has the highest priority
            load_env_from_file(option_env!("RUST_DEV_ENV_FILE").unwrap_or(".env.local"))
        } else if #[cfg(all(feature = "production", not(feature = "local")))] {
            // "production" feature has second priority (when "local" isn't present)
            load_env_from_file(option_env!("RUST_PROD_ENV_FILE").unwrap_or(".env.production"))
        } else if #[cfg(not(debug_assertions))] {
            // Release builds without explicit features use production env
            load_env_from_file(option_env!("RUST_PROD_ENV_FILE").unwrap_or(".env.production"))
        } else {
            // Debug builds without explicit features use local env
            load_env_from_file(option_env!("RUST_DEV_ENV_FILE").unwrap_or(".env.local"))
        }
    }
}
