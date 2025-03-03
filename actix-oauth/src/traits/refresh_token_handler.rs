//! Refresh Token grant type handler for OAuth2.
//!
//! This module provides the [`RefreshTokenHandler`] trait for implementing the
//! Refresh Token grant type as specified in RFC 6749, Section 6.

use crate::handler::HandlerReturn;
use crate::types::{ClientId, ClientSecret, RefreshToken};
use actix_web::HttpRequest;

/// Handler for the OAuth2 Refresh Token grant type.
///
/// This trait is implemented for types that can process OAuth2 Refresh Token
/// grant requests according to RFC 6749, Section 6. This grant type is used to
/// obtain a new access token when the current one has expired, without requiring
/// the resource owner to re-authenticate.
///
/// # Parameters
///
/// * [`HttpRequest`] - The incoming HTTP request containing headers and context
/// * [`Option<ClientId>`](crate::ClientId) - Optional client identifier (may be required depending on configuration)
/// * [`Option<ClientSecret>`](crate::ClientSecret) - Optional client secret (may be required depending on configuration)
/// * [`RefreshToken`] - The refresh token previously issued to the client
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
/// use actix_oauth::traits::RefreshTokenHandler;
/// use actix_oauth::handler::HandlerReturn;
/// use actix_oauth::types::{ClientId, ClientSecret, RefreshToken};
/// use actix_oauth::dto::token_response::TokenResponse;
/// use actix_oauth::error::Oauth2ErrorType;
/// use actix_web::HttpRequest;
///
/// async fn handle_refresh_token(
///     _req: HttpRequest,
///     client_id: Option<ClientId>,
///     client_secret: Option<ClientSecret>,
///     refresh_token: RefreshToken,
/// ) -> HandlerReturn {
///     // 1. Validate the refresh token
///     if !validate_refresh_token(&refresh_token.secret()) {
///         return Err(Oauth2ErrorType::InvalidGrant);
///     }
///
///     // 2. Authenticate the client if client_id and client_secret are provided
///     if let (Some(id), Some(secret)) = (client_id, client_secret) {
///         if !validate_client(&id, &secret.secret()) {
///             return Err(Oauth2ErrorType::InvalidClient);
///         }
///     }
///
///     // 3. Generate a new access token and optionally a new refresh token
///
///     // Example successful response
///     Ok(TokenResponse {
///         access_token: "new_access_token".to_string().into(),
///         token_type: "bearer".to_string().into(),
///         expires_in: Some(3600).into(),
///         refresh_token: Some("new_refresh_token".to_string()).into(),
///     })
/// }
///
/// fn validate_refresh_token(_token: &str) -> bool {
///     // Your implementation to validate the refresh token
///     // Check expiration, validity, etc.
///     true
/// }
///
/// fn validate_client(_id: &str, _secret: &str) -> bool {
///     // Your implementation to validate the client credentials
///     true
/// }
/// ```
#[diagnostic::on_unimplemented(
    note = "Consider creating a custom handler that processes refresh token requests",
    message = "`{Self}` must be able to process refresh token requests",
    label = "this type doesn't implement the required function signature for handling refresh token grants"
)]
pub trait RefreshTokenHandler:
    AsyncFn(HttpRequest, Option<ClientId>, Option<ClientSecret>, RefreshToken) -> HandlerReturn
    + Send
    + Sync
    + Clone
    + 'static
{
}

impl<T> RefreshTokenHandler for T where
    T: AsyncFn(HttpRequest, Option<ClientId>, Option<ClientSecret>, RefreshToken) -> HandlerReturn
        + Send
        + Sync
        + Clone
        + 'static
{
}
