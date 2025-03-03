//! Authorization endpoint handler for OAuth2.
//!
//! This module provides the [`AuthorizationHandler`] trait for implementing the
//! authorization endpoint functionality as specified in RFC 6749, Section 3.1.

use crate::dto::AuthorizationRequest;
use crate::handler::AuthorizationReturn;
use actix_web::HttpRequest;

/// Handler for the OAuth2 Authorization endpoint.
///
/// This trait is implemented for types that can process OAuth2 authorization requests
/// according to RFC 6749, Section 3.1. The authorization endpoint is used to interact
/// with the resource owner and obtain an authorization grant, typically by redirecting
/// the resource owner's user-agent to the authorization server's UI.
///
/// # Parameters
///
/// * [`HttpRequest`] - The incoming HTTP request containing headers and context
/// * [`AuthorizationRequest`] - The authorization request parameters
///
/// # Returns
///
/// * [`AuthorizationReturn`] - A Result containing either a [HttpResponse](actix_web::HttpResponse) or an [Oauth2ErrorType](crate::error::Oauth2ErrorType)
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
/// use actix_oauth::traits::AuthorizationHandler;
/// use actix_oauth::dto::AuthorizationRequest;
/// use actix_oauth::handler::AuthorizationReturn;
/// use actix_oauth::error::Oauth2ErrorType;
/// use actix_web::{HttpRequest, HttpResponse};
///
/// async fn handle_authorization(
///     _req: HttpRequest,
///     auth_req: AuthorizationRequest,
/// ) -> AuthorizationReturn {
///     // 1. Validate the request parameters
///     if auth_req.client_id.is_empty() {
///         return Err(Oauth2ErrorType::InvalidRequest);
///     }
///
///     // 2. Authenticate the resource owner (typically through a login form)
///     // 3. Ask for consent if necessary
///     // 4. Generate an authorization code or token depending on response_type
///     // 5. Redirect to the specified redirect_uri
///
///     // Example response for an authorization code flow
///     let redirect_uri = format!(
///         "{}?code=AUTHORIZATION_CODE&state={}",
///         auth_req.redirect_uri,
///         auth_req.state.unwrap_or_default()
///     );
///
///     Ok(HttpResponse::Found()
///         .append_header(("Location", redirect_uri))
///         .finish())
/// }
/// ```
#[diagnostic::on_unimplemented(
    note = "Consider creating a custom handler that processes authorization requests",
    message = "`{Self}` must be able to process authorization endpoint requests",
    label = "this type doesn't implement the required function signature for handling authorization requests"
)]
pub trait AuthorizationHandler:
    AsyncFn(HttpRequest, AuthorizationRequest) -> AuthorizationReturn + Send + Sync + Clone + 'static
{
}

impl<T> AuthorizationHandler for T where
    T: AsyncFn(HttpRequest, AuthorizationRequest) -> AuthorizationReturn
        + Send
        + Sync
        + Clone
        + 'static
{
}
