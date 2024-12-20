use crate::dto::token_response::TokenResponse;
use crate::dto::{Oauth2ErrorResponses, OauthRequest};
use crate::error::Oauth2ErrorType;
use crate::types::{AuthorizationCode, ClientId, ClientSecret, Password, RefreshToken, Username};
use actix_web::dev::{AppService, HttpServiceFactory};
use actix_web::{guard, web, HttpRequest};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use utoipa::__dev::{SchemaReferences, Tags};
use utoipa::openapi::path::{Operation, Parameter};
use utoipa::openapi::{HttpMethod, RefOr, Required, Responses, Schema};
use utoipa::{IntoResponses, PartialSchema, Path, ToResponse, ToSchema};

pub type HandlerReturn = Result<TokenResponse, Oauth2ErrorType>;
pub type HandlerFuture = Pin<Box<dyn Future<Output = HandlerReturn> + Send + 'static>>;

pub(crate) type HandlerField<H> = Arc<H>;

pub struct Oauth2Handler {
    password_grant_handler:
        Option<HandlerField<dyn Fn(HttpRequest, Username, Password) -> HandlerFuture>>,
    authorization_code_grant_handler: Option<
        HandlerField<
            dyn Fn(HttpRequest, AuthorizationCode, String, ClientId, ClientSecret) -> HandlerFuture,
        >,
    >,
    client_credentials_grant_handler:
        Option<HandlerField<dyn Fn(HttpRequest, ClientId, ClientSecret) -> HandlerFuture>>,
    refresh_token_handler: Option<
        HandlerField<
            dyn Fn(
                HttpRequest,
                Option<ClientId>,
                Option<ClientSecret>,
                RefreshToken,
            ) -> HandlerFuture,
        >,
    >,
}

pub struct Oauth2HandlerBuilder {
    password_grant_handler:
        Option<HandlerField<dyn Fn(HttpRequest, Username, Password) -> HandlerFuture>>,
    authorization_code_grant_handler: Option<
        HandlerField<
            dyn Fn(HttpRequest, AuthorizationCode, String, ClientId, ClientSecret) -> HandlerFuture,
        >,
    >,
    client_credentials_grant_handler:
        Option<HandlerField<dyn Fn(HttpRequest, ClientId, ClientSecret) -> HandlerFuture>>,
    refresh_token_handler: Option<
        HandlerField<
            dyn Fn(
                HttpRequest,
                Option<ClientId>,
                Option<ClientSecret>,
                RefreshToken,
            ) -> HandlerFuture,
        >,
    >,
}

impl Oauth2HandlerBuilder {
    pub fn new() -> Self {
        Self {
            password_grant_handler: None,
            authorization_code_grant_handler: None,
            client_credentials_grant_handler: None,
            refresh_token_handler: None,
        }
    }

    pub fn password_handler(
        mut self,
        handler: impl Fn(HttpRequest, Username, Password) -> HandlerFuture + 'static,
    ) -> Self {
        self.password_grant_handler = Some(Arc::new(handler));
        self
    }

    pub fn authorization_code_handler(
        mut self,
        handler: impl Fn(HttpRequest, AuthorizationCode, String, ClientId, ClientSecret) -> HandlerFuture
            + 'static,
    ) -> Self {
        self.authorization_code_grant_handler = Some(Arc::new(handler));
        self
    }

    pub fn client_credentials_handler(
        mut self,
        handler: impl Fn(HttpRequest, ClientId, ClientSecret) -> HandlerFuture + 'static,
    ) -> Self {
        self.client_credentials_grant_handler = Some(Arc::new(handler));
        self
    }

    pub fn refresh_handler(
        mut self,
        handler: impl Fn(HttpRequest, Option<ClientId>, Option<ClientSecret>, RefreshToken) -> HandlerFuture
            + 'static,
    ) -> Self {
        self.refresh_token_handler = Some(Arc::new(handler));
        self
    }

    pub fn build(self) -> Oauth2Handler {
        Oauth2Handler {
            password_grant_handler: self.password_grant_handler,
            authorization_code_grant_handler: self.authorization_code_grant_handler,
            client_credentials_grant_handler: self.client_credentials_grant_handler,
            refresh_token_handler: self.refresh_token_handler,
        }
    }
}

impl HttpServiceFactory for Oauth2Handler {
    fn register(self, config: &mut AppService) {
        let password_handler = self.password_grant_handler;
        let authorization_code_handler = self.authorization_code_grant_handler;
        let client_credentials_handler = self.client_credentials_grant_handler;
        let refresh_handler = self.refresh_token_handler;

        let handler = move |req: HttpRequest, web::Form(oauth_req): web::Form<OauthRequest>| {
            let password_handler = password_handler.clone();
            let authorization_code_handler = authorization_code_handler.clone();
            let client_credentials_handler = client_credentials_handler.clone();

            let refresh_handler = refresh_handler.clone();

            async move {
                match oauth_req {
                    OauthRequest::Password { username, password } => {
                        if let Some(method) = password_handler {
                            method(req, username, password).await
                        } else {
                            Err(Oauth2ErrorType::UnsupportedGrantType)
                        }
                    }
                    OauthRequest::AuthorizationCode {
                        code,
                        redirect_url,
                        client_id,
                        client_secret,
                    } => {
                        if let Some(method) = authorization_code_handler {
                            method(req, code, redirect_url, client_id, client_secret).await
                        } else {
                            Err(Oauth2ErrorType::UnsupportedGrantType)
                        }
                    }
                    OauthRequest::ClientCredentials {
                        client_id,
                        client_secret,
                    } => {
                        if let Some(method) = client_credentials_handler {
                            method(req, client_id, client_secret).await
                        } else {
                            Err(Oauth2ErrorType::UnsupportedGrantType)
                        }
                    }
                    OauthRequest::RefreshToken {
                        client_id,
                        client_secret,
                        refresh_token,
                    } => {
                        if let Some(method) = refresh_handler {
                            method(req, client_id, client_secret, refresh_token).await
                        } else {
                            Err(Oauth2ErrorType::UnsupportedGrantType)
                        }
                    }
                }
            }
        };

        let resource = actix_web::Resource::new("/oauth/token")
            .guard(guard::Post())
            .to(handler);

        HttpServiceFactory::register(resource, config);
    }
}

#[doc(hidden)]
#[allow(non_camel_case_types)]
pub struct __path_Oauth2Handler;

impl Path for __path_Oauth2Handler {
    fn methods() -> Vec<HttpMethod> {
        vec![HttpMethod::Post]
    }

    fn path() -> String {
        String::from("/oauth/token")
    }

    fn operation() -> Operation {
        let json_schema = OauthRequest::schema();
        let (_, token_res) = TokenResponse::response();
        let error = Oauth2ErrorResponses::responses();

        let mut responses = Responses::new();
        responses.responses = error;
        responses.responses.insert("200".to_string(), token_res);

        Operation::builder()
            .tag("Oauth")
            .summary(Some("Token"))
            .description(Some("Endpoint to authenticate using oauth2"))
            .parameter(
                Parameter::builder()
                    .schema(Some(json_schema))
                    .name(OauthRequest::name())
                    .required(Required::True)
                    .description(Some("A request to get a OAuth2 Access Token")),
            )
            .responses(responses)
            .build()
    }
}

impl SchemaReferences for __path_Oauth2Handler {
    fn schemas(_schemas: &mut Vec<(String, RefOr<Schema>)>) {}
}

impl<'a> Tags<'a> for __path_Oauth2Handler {
    fn tags() -> Vec<&'a str> {
        vec!["Oauth"]
    }
}
