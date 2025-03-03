use crate::dto::{AuthorizationRequest, OauthRequest};
use crate::handler::HandlerReturn;
use crate::traits::authorization_handler::AuthorizationHandler;
use actix_web::dev::{AppService, HttpServiceFactory};
use actix_web::web::post;
use actix_web::{HttpRequest, web};
use derive_more::{AsMut, AsRef, Deref, DerefMut};

#[derive(Debug, Clone, AsRef, AsMut, Deref, DerefMut)]
pub struct OAuth2ManagerService<T: OAuth2Manager>(T);

pub trait OAuth2Manager: Clone + 'static {
    async fn token_handler(&self, req: HttpRequest, oauth_req: OauthRequest) -> HandlerReturn;
    fn authorization_handler(&self) -> impl AuthorizationHandler;
}

impl<T: OAuth2Manager> HttpServiceFactory for OAuth2ManagerService<T> {
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

pub trait OAuth2ManagerExt: OAuth2Manager + Sized {
    fn into_service(self) -> OAuth2ManagerService<Self> {
        OAuth2ManagerService(self)
    }
}

impl<T: OAuth2Manager> OAuth2ManagerExt for T {}
