//! Password grant type handler for OAuth2.
//!
//! This module provides the [`PasswordHandler`] trait for implementing the
//! Resource Owner Password Credentials grant type as specified in RFC 6749, Section 4.3.

use crate::handler::HandlerReturn;
use crate::types::{Password, Username};
use actix_web::HttpRequest;

/// Handler for the OAuth2 Resource Owner Password Credentials grant type.
///
/// This trait is implemented for types that can process OAuth2 Password grant requests
/// according to RFC 6749, Section 4.3. This grant type is used when the application
/// has a high degree of trust with the resource owner (e.g., first-party applications).
///
/// # Security Considerations
///
/// This grant type requires the client application to collect the resource owner's
/// username and password directly. As such, it should only be used when strictly
/// necessary and when other, more secure flows (like authorization code) are not feasible.
///
/// # Parameters
///
/// * [`HttpRequest`] - The incoming HTTP request containing headers and context
/// * [`Username`] - The resource owner's username
/// * [`Password`] - The resource owner's password
///
/// # Returns
///
/// * [`HandlerReturn`] - A Result containing either a [TokenResponse](crate::TokenResponse) or an [Oauth2ErrorType](crate::error::Oauth2ErrorType)
///
/// # Implementation
///
/// This trait is automatically implemented for any type that satisfies the required
/// function signature. You don't need to manually implement the trait, just provide
/// a function with the correct signature.
///
/// # Example
///
/// ```
/// use actix_oauth::traits::PasswordHandler;
/// use actix_oauth::handler::HandlerReturn;
/// use actix_oauth::types::{Username, Password};
/// use actix_oauth::dto::token_response::TokenResponse;
/// use actix_oauth::error::Oauth2ErrorType;
/// use actix_web::HttpRequest;
///
/// async fn handle_password(
///     _req: HttpRequest,
///     username: Username,
///     password: Password,
/// ) -> HandlerReturn {
///     // 1. Authenticate the user with the provided credentials
///     if !validate_user_credentials(&username, &password) {
///         return Err(Oauth2ErrorType::InvalidGrant);
///     }
///
///     // 2. Generate access token and optional refresh token
///
///     // Example successful response
///     Ok(TokenResponse {
///         access_token: "generated_token".to_string().into(),
///         token_type: "bearer".to_string().into(),
///         expires_in: Some(3600).into(),
///         refresh_token: Some("refresh_token".to_string()).into(),
///     })
/// }
///
/// fn validate_user_credentials(_username: &str, _password: &str) -> bool {
///     // Your implementation to validate user credentials against your database
///     // For example, hash the password and compare with stored hash
///     true
/// }
/// ```
#[diagnostic::on_unimplemented(
    note = "Consider creating a custom handler that processes password grant requests",
    message = "`{Self}` must be able to process resource owner password credentials requests",
    label = "this type doesn't implement the required function signature for handling password grants"
)]
pub trait PasswordHandler:
    AsyncFn(HttpRequest, Username, Password) -> HandlerReturn + Send + Sync + Clone + 'static
{
}

impl<T> PasswordHandler for T where
    T: AsyncFn(HttpRequest, Username, Password) -> HandlerReturn + Send + Sync + Clone + 'static
{
}
