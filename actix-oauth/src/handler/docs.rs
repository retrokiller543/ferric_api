use crate::dto::{AuthorizationRequest, Oauth2ErrorResponses, OauthRequest, TokenResponse};
use actix_web::{HttpResponse, Responder, post, web};

/// Exchange credentials for an access token.
///
/// Supports Form data, Json or query params.
#[utoipa::path(
    tags = ["OAuth"],
    request_body(
        description = "The different OAuth2 flows that can be parsed but not guaranteed to be implemented, *Note that it can be sent as a query param as well but its not recommended*",
        content(
            (OauthRequest = "application/json"),
            (OauthRequest = "application/x-www-form-urlencoded")
        )
    ),
    responses(
        Oauth2ErrorResponses,
        (status = 200, description = "Successfully got a access token", body = TokenResponse)
    )
)]
#[allow(dead_code)]
#[post("/oauth/token")]
async fn token(
    _: actix_web::Either<
        actix_web::Either<web::Form<OauthRequest>, web::Json<OauthRequest>>,
        web::Query<OauthRequest>,
    >,
) -> impl Responder {
    HttpResponse::Ok()
}

/// Exchange credentials for short-lived code
///
/// The code can later be exchanged for a long-lived token
#[utoipa::path(
    tags = ["OAuth"],
    params(AuthorizationRequest),
    responses(
        Oauth2ErrorResponses,
        (status = 304, description = "Returns the code in the query params")
    )
)]
#[allow(dead_code)]
#[post("/oauth/authorize")]
async fn authorize(_: web::Query<AuthorizationRequest>) -> impl Responder {
    HttpResponse::Ok()
}
