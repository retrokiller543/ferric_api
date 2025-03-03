//! Client Credentials grant type handler for OAuth2.
//!
//! This module provides the [`ClientCredentialsHandler`] trait for implementing the
//! Client Credentials grant type as specified in RFC 6749, Section 4.4.

use crate::handler::HandlerReturn;
use crate::types::{ClientId, ClientSecret};
use actix_web::HttpRequest;

/// Handler for the OAuth2 Client Credentials grant type.
///
/// This trait is implemented for types that can process OAuth2 Client Credentials
/// grant requests according to RFC 6749, Section 4.4. This grant type is used for
/// machine-to-machine authentication where no user interaction is involved.
///
/// # Parameters
///
/// * [`HttpRequest`] - The incoming HTTP request containing headers and context
/// * [`ClientId`] - The client identifier
/// * [`ClientSecret`] - The client secret for authentication
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
/// use actix_oauth::traits::ClientCredentialsHandler;
/// use actix_oauth::handler::HandlerReturn;
/// use actix_oauth::types::{ClientId, ClientSecret};
/// use actix_oauth::dto::token_response::TokenResponse;
/// use actix_oauth::error::Oauth2ErrorType;
/// use actix_web::HttpRequest;
///
/// async fn handle_client_credentials(
///     _req: HttpRequest,
///     client_id: ClientId,
///     client_secret: ClientSecret,
/// ) -> HandlerReturn {
///     // 1. Authenticate the client (verify client_id and client_secret)
///     if !validate_client(client_id.as_str(), client_secret.secret().as_str()) {
///         return Err(Oauth2ErrorType::InvalidClient);
///     }
///
///     // 2. Generate access token (typically without a refresh token)
///
///     // Example successful response
///     Ok(TokenResponse {
///         access_token: "generated_token".to_string().into(),
///         token_type: "bearer".to_string().into(),
///         expires_in: Some(3600).into(),
///         refresh_token: None.into(), // Usually no refresh token for client credentials
///     })
/// }
///
/// fn validate_client(_id: &str, _secret: &str) -> bool {
///     // Your implementation to validate the client credentials
///     true
/// }
/// ```
#[diagnostic::on_unimplemented(
    note = "Consider creating a custom handler that processes client credentials grant requests",
    message = "`{Self}` must be able to process client credentials requests",
    label = "this type doesn't implement the required function signature for handling client credentials grants"
)]
pub trait ClientCredentialsHandler:
    AsyncFn(HttpRequest, ClientId, ClientSecret) -> HandlerReturn + Send + Sync + Clone + 'static
{
}

impl<T> ClientCredentialsHandler for T where
    T: AsyncFn(HttpRequest, ClientId, ClientSecret) -> HandlerReturn
        + Send
        + Sync
        + Clone
        + 'static
{
}
