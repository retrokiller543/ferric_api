//! Authorization Code grant type handler for OAuth2.
//!
//! This module provides the [`AuthCodeHandler`] trait for implementing the
//! Authorization Code grant type as specified in RFC 6749, Section 4.1.

use crate::handler::HandlerReturn;
use crate::types::{AuthorizationCode, ClientId, ClientSecret, RedirectUri};
use actix_web::HttpRequest;

/// Handler for the OAuth2 Authorization Code grant type.
///
/// This trait is implemented for types that can process OAuth2 Authorization Code
/// grant requests according to RFC 6749, Section 4.1. This grant type is used for
/// applications such as server-side web applications where the client secret can
/// be kept confidential.
///
/// # Parameters
///
/// * [`HttpRequest`] - The incoming HTTP request containing headers and context
/// * [`AuthorizationCode`] - The authorization code received from the authorization server
/// * [`RedirectUri`] - The redirect URI that was used in the authorization request
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
/// use actix_oauth::traits::AuthCodeHandler;
/// use actix_oauth::handler::HandlerReturn;
/// use actix_oauth::types::{AuthorizationCode, ClientId, ClientSecret, RedirectUri};
/// use actix_oauth::dto::token_response::TokenResponse;
/// use actix_oauth::error::Oauth2ErrorType;
/// use actix_web::HttpRequest;
///
/// async fn handle_auth_code(
///     _req: HttpRequest,
///     code: AuthorizationCode,
///     redirect_uri: RedirectUri,
///     client_id: ClientId,
///     client_secret: ClientSecret,
/// ) -> HandlerReturn {
///     // 1. Validate the authorization code
///     // 2. Verify the redirect URI matches the one used for the authorization request
///     // 3. Authenticate the client (verify client_id and client_secret)
///     // 4. Generate access token, refresh token, etc.
///
///     if code.secret().is_empty() {
///         return Err(Oauth2ErrorType::InvalidGrant);
///     }
///
///     // Example successful response
///     Ok(TokenResponse {
///         access_token: "generated_token".to_string().into(),
///         token_type: "bearer".to_string().into(),
///         expires_in: Some(3600).into(),
///         refresh_token: Some("refresh_token".to_string()).into(),
///     })
/// }
/// ```
#[diagnostic::on_unimplemented(
    note = "Consider creating a custom handler that processes authorization code grant requests",
    message = "`{Self}` must be able to process authorization code requests",
    label = "this type doesn't implement the required function signature for handling authorization code grants"
)]
pub trait AuthCodeHandler:
    AsyncFn(HttpRequest, AuthorizationCode, RedirectUri, ClientId, ClientSecret) -> HandlerReturn
    + Send
    + Sync
    + Clone
    + 'static
{
}

impl<T> AuthCodeHandler for T where
    T: AsyncFn(
            HttpRequest,
            AuthorizationCode,
            RedirectUri,
            ClientId,
            ClientSecret,
        ) -> HandlerReturn
        + Send
        + Sync
        + Clone
        + 'static
{
}
