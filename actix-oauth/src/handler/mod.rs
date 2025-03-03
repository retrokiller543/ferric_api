//! OAuth2 implementation providing standard grant types according to the OAuth2 specification.
//!
//! This module contains the core handler for OAuth2 token requests and authorization
//! endpoints. It implements the following grant types:
//! - Password grant
//! - Authorization code grant
//! - Client credentials grant
//! - Refresh token grant
//!
//! # Usage
//!
//! ```
//! use actix_oauth::handler::OAuth2HandlerBuilder;
//! use actix_oauth::handler::default::{NotImplementedPasswordHandler, NotImplementedAuthCodeHandler};
//!
//! // Create a custom OAuth2 handler with specific implementations
//! let oauth2_handler = OAuth2HandlerBuilder::new()
//!     .password_handler(NotImplementedPasswordHandler)
//!     .authorization_code_handler(NotImplementedAuthCodeHandler)
//!     .build();
//!
//! // Use with actix-web
//! use actix_web::{App, web};
//! let app = App::new()
//!     .service(oauth2_handler);
//! ```

mod builder;
pub mod default;
pub(crate) mod docs;
pub use builder::OAuth2HandlerBuilder;
use default::*;

use crate::dto::OauthRequest;
use crate::dto::token_response::TokenResponse;
use crate::error::Oauth2ErrorType;
use crate::traits::*;
use actix_web::dev::{AppService, HttpServiceFactory};
use actix_web::{HttpRequest, HttpResponse};

/// Result type for token endpoint operations
pub type HandlerReturn = Result<TokenResponse, Oauth2ErrorType>;

/// Result type for authorization endpoint operations
pub type AuthorizationReturn = Result<HttpResponse, Oauth2ErrorType>;

/// Main OAuth2 handler that processes token and authorization requests.
///
/// This struct implements the core functionality required by the OAuth2 specification,
/// handling different grant types through specialized handler implementations.
/// Each handler can be customized through the [`OAuth2HandlerBuilder`].
///
/// # Type Parameters
///
/// * `PH` - Password grant handler, must implement [`PasswordHandler`] trait
/// * `AH` - Authorization code grant handler, must implement [`AuthCodeHandler`] trait
/// * `CH` - Client credentials grant handler, must implement [`ClientCredentialsHandler`] trait
/// * `RH` - Refresh token grant handler, must implement [`RefreshTokenHandler`] trait
/// * `AuthH` - Authorization endpoint handler, must implement [`AuthorizationHandler`] trait
///
/// # Examples
///
/// ```
/// use actix_oauth::handler::{default::NotImplementedPasswordHandler, OAuth2HandlerBuilder, OAuth2Handler};
///
/// // Create a handler with default implementations (which will return errors)
/// let default_handler = OAuth2Handler::default();
///
/// // Create a custom handler with specific implementations
/// let custom_handler = OAuth2HandlerBuilder::new()
///     .password_handler(NotImplementedPasswordHandler)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct OAuth2Handler<
    PH = NotImplementedPasswordHandler,
    AH = NotImplementedAuthCodeHandler,
    CH = NotImplementedClientCredentialsHandler,
    RH = NotImplementedRefreshTokenHandler,
    AuthH = NotImplementedAuthorizationHandler,
> where
    PH: PasswordHandler,
    AH: AuthCodeHandler,
    CH: ClientCredentialsHandler,
    RH: RefreshTokenHandler,
    AuthH: AuthorizationHandler,
{
    password_grant_handler: PH,
    authorization_code_grant_handler: AH,
    client_credentials_grant_handler: CH,
    refresh_token_handler: RH,
    authorization_handler: AuthH,
}

impl Default for OAuth2Handler {
    /// Creates a new OAuth2Handler with default implementations.
    ///
    /// The default implementations will return appropriate error responses
    /// when called, as they don't provide actual functionality.
    /// Use [`OAuth2HandlerBuilder`] to customize the handlers.
    ///
    /// # Returns
    ///
    /// A new [`OAuth2Handler`] with default (non-functional) handler implementations.
    #[inline(always)]
    fn default() -> Self {
        OAuth2HandlerBuilder::new().build()
    }
}

impl<PH, AH, CH, RH, AuthH> OAuth2Manager for OAuth2Handler<PH, AH, CH, RH, AuthH>
where
    PH: PasswordHandler,
    AH: AuthCodeHandler,
    CH: ClientCredentialsHandler,
    RH: RefreshTokenHandler,
    AuthH: AuthorizationHandler,
{
    /// Processes token requests according to the OAuth2 specification.
    ///
    /// This method routes the request to the appropriate handler based on the
    /// grant type specified in the request.
    ///
    /// # Parameters
    ///
    /// * `req` - The HTTP request containing headers and other context
    /// * `oauth_req` - The parsed OAuth2 request data
    ///
    /// # Returns
    ///
    /// A `HandlerReturn` containing either a `TokenResponse` or an `Oauth2ErrorType`
    ///
    /// # Tracing
    ///
    /// This method is instrumented with tracing at debug level, skipping all parameters
    /// to avoid logging sensitive information.
    #[tracing::instrument(skip_all, level = "debug")]
    async fn token_handler(&self, req: HttpRequest, oauth_req: OauthRequest) -> HandlerReturn {
        match oauth_req {
            OauthRequest::Password { username, password } => {
                self.password_grant_handler
                    .async_call((req, username, password))
                    .await
            }
            OauthRequest::AuthorizationCode {
                code,
                redirect_uri,
                client_id,
                client_secret,
            } => {
                self.authorization_code_grant_handler
                    .async_call((req, code, redirect_uri, client_id, client_secret))
                    .await
            }
            OauthRequest::ClientCredentials {
                client_id,
                client_secret,
            } => {
                self.client_credentials_grant_handler
                    .async_call((req, client_id, client_secret))
                    .await
            }
            OauthRequest::RefreshToken {
                client_id,
                client_secret,
                refresh_token,
            } => {
                self.refresh_token_handler
                    .async_call((req, client_id, client_secret, refresh_token))
                    .await
            }
        }
    }

    /// Returns the authorization handler for processing authorization requests.
    ///
    /// # Returns
    ///
    /// A clone of the configured authorization handler implementation.
    #[inline(always)]
    fn authorization_handler(&self) -> impl AuthorizationHandler {
        self.authorization_handler.clone()
    }
}

impl<PH, AH, CH, RH, AuthH> HttpServiceFactory for OAuth2Handler<PH, AH, CH, RH, AuthH>
where
    PH: PasswordHandler,
    AH: AuthCodeHandler,
    CH: ClientCredentialsHandler,
    RH: RefreshTokenHandler,
    AuthH: AuthorizationHandler,
{
    /// Registers the OAuth2 handler with the Actix web application.
    ///
    /// This allows the handler to be used as a service in an Actix web application.
    ///
    /// # Parameters
    ///
    /// * `config` - The Actix web application service configuration
    fn register(self, config: &mut AppService) {
        self.into_service().register(config)
    }
}
