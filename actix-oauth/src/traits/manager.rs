//! OAuth2 Manager implementation and service registration.
//!
//! This module provides the core [`OAuth2Manager`] trait and related functionality
//! for registering OAuth2 endpoints with an Actix web application.

use crate::dto::{AuthorizationRequest, OauthRequest};
use crate::handler::HandlerReturn;
use crate::traits::authorization_handler::AuthorizationHandler;
use actix_web::dev::{AppService, HttpServiceFactory};
use actix_web::web::post;
use actix_web::{HttpRequest, web};
use derive_more::{AsMut, AsRef, Deref, DerefMut};

/// Service wrapper for OAuth2Manager implementations.
///
/// This struct wraps an [`OAuth2Manager`] implementation and provides the Actix
/// web service registration functionality. It handles routing, request parsing,
/// and connecting the HTTP interface to the OAuth2 implementation.
///
/// This type is typically created using the [`OAuth2ManagerExt::into_service`] method.
#[derive(Debug, Clone, AsRef, AsMut, Deref, DerefMut)]
pub struct OAuth2ManagerService<T: OAuth2Manager>(T);

/// Core trait for OAuth2 functionality.
///
/// This trait defines the main functionality required for an OAuth2 service,
/// including the token endpoint handler and authorization endpoint handler.
/// It also provides integration with Actix web through the [`OAuth2ManagerExt::into_service`]
/// implementation.
///
/// # Implementation
///
/// Implementers of this trait need to provide:
///
/// 1. A token handler that processes all OAuth2 token requests
/// 2. An authorization handler that processes authorization requests
///
/// The trait also requires that the implementation is [`Clone`] and [`'static`].
///
/// # Example
///
/// ```
/// use actix_oauth::{traits::OAuth2ManagerExt, handler::{OAuth2Handler, OAuth2HandlerBuilder}};
/// use actix_web::{App, web};
///
/// // Create a custom OAuth2 handler
/// let oauth2_handler = OAuth2HandlerBuilder::new()
///     .build();
///
/// // Use with actix-web
/// let app = App::new()
///     .service(web::scope("/oauth2").service(oauth2_handler.into_service()));
/// ```
#[diagnostic::on_unimplemented(
    note = "Implement this trait to create a custom OAuth2 manager",
    message = "`{Self}` must implement OAuth2Manager to be usable with the OAuth2 service",
    label = "this type doesn't implement the required OAuth2Manager methods"
)]
pub trait OAuth2Manager: Clone + 'static {
    /// Handles OAuth2 token requests.
    ///
    /// This method processes token requests for all supported grant types.
    /// It should dispatch the request to the appropriate handler based on
    /// the grant type and parameters.
    ///
    /// # Parameters
    ///
    /// * `req` - The HTTP request containing headers and context
    /// * `oauth_req` - The parsed OAuth2 request data
    ///
    /// # Returns
    ///
    /// * `HandlerReturn` - A Result containing either a TokenResponse or an OAuth2 error
    async fn token_handler(&self, req: HttpRequest, oauth_req: OauthRequest) -> HandlerReturn;
    /// Returns the authorization handler.
    ///
    /// This method should return a handler for authorization requests that
    /// implements the [`AuthorizationHandler`] trait.
    ///
    /// # Returns
    ///
    /// * `impl AuthorizationHandler` - The authorization handler implementation
    fn authorization_handler(&self) -> impl AuthorizationHandler;
}

impl<T: OAuth2Manager> HttpServiceFactory for OAuth2ManagerService<T> {
    /// Registers the OAuth2 endpoints with an Actix web application.
    ///
    /// This method sets up the following routes:
    /// - POST /oauth/token - Token endpoint for all grant types
    /// - POST /oauth/authorize - Authorization endpoint
    ///
    /// The token endpoint accepts form, JSON, or query parameters.
    ///
    /// # Parameters
    ///
    /// * `config` - The Actix web application service configuration
    fn register(self, config: &mut AppService) {
        let handler = self;

        let token_handler = {
            let handler = handler.clone();

            move |req: HttpRequest,
                  oauth_req: actix_web::Either<
                actix_web::Either<web::Form<OauthRequest>, web::Json<OauthRequest>>,
                web::Query<OauthRequest>,
            >| {
                let oauth_req = match oauth_req {
                    web::Either::Left(web::Either::Left(web::Form(oauth_request))) => oauth_request,
                    web::Either::Left(web::Either::Right(web::Json(oauth_request))) => {
                        oauth_request
                    }
                    web::Either::Right(web::Query(oauth_request)) => oauth_request,
                };
                let handler = handler.clone();
                async move { handler.token_handler(req, oauth_req).await }
            }
        };

        let authorization_handler = {
            let auth_handler = handler.authorization_handler().clone();

            move |req: HttpRequest,
                  web::Query(authorization_request): web::Query<AuthorizationRequest>| {
                let auth_handler = auth_handler.clone();

                async move { auth_handler.async_call((req, authorization_request)).await }
            }
        };

        let scope = web::scope("/oauth")
            .route("/token", post().to(token_handler))
            .route("/authorize", post().to(authorization_handler));

        HttpServiceFactory::register(scope, config);
    }
}

/// Extension trait for OAuth2Manager implementations.
///
/// This trait provides convenience methods for working with OAuth2Manager
/// implementations, such as converting them into services.
pub trait OAuth2ManagerExt: OAuth2Manager + Sized {
    /// Converts the OAuth2Manager into a service that can be registered with Actix.
    ///
    /// This method wraps the OAuth2Manager in an [`OAuth2ManagerService`]
    /// for integration with Actix web.
    ///
    /// # Returns
    ///
    /// * `OAuth2ManagerService<Self>` - The service wrapper for the manager
    ///
    /// # Example
    ///
    /// ```
    /// use actix_oauth::handler::OAuth2Handler;
    /// use actix_oauth::traits::OAuth2ManagerExt;
    /// use actix_web::{App, web};
    ///
    /// let handler = OAuth2Handler::default();
    /// let service = handler.into_service();
    ///
    /// let app = App::new()
    ///     .service(web::scope("/api").service(service));
    /// ```
    fn into_service(self) -> OAuth2ManagerService<Self> {
        OAuth2ManagerService(self)
    }
}

impl<T: OAuth2Manager> OAuth2ManagerExt for T {}
