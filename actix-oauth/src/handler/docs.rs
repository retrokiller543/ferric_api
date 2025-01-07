use crate::dto::{AuthorizationRequest, Oauth2ErrorResponses, OauthRequest, TokenResponse};

/// Exchange credentials for an access token.
///
/// Supports Form data, Json or query params.
#[utoipa::path(
    post,
    path = "/oauth/token",
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
        (status = 200, response = TokenResponse)
    )
)]
#[allow(dead_code)]
fn token() {}

/// Exchange credentials for short-lived code
///
/// The code can later be exchanged for a long-lived token
#[utoipa::path(
    post,
    path = "/oauth/authorize",
    tags = ["OAuth"],
    params(AuthorizationRequest),
    responses(
        Oauth2ErrorResponses,
        (status = 304, description = "Returns the code in the query params")
    )
)]
#[allow(dead_code)]
fn authorize() {}
