use crate::repositories::users::get_users_repository;
use crate::services::oauth::create_token_response;
use actix_oauth::error::Oauth2ErrorType;
use actix_oauth::handler::HandlerReturn;
use actix_oauth::types::{Password, Username};
use actix_web::HttpRequest;

#[inline]
#[actix_oauth::oauth]
#[tracing::instrument(skip_all, level = "debug")]
pub(crate) async fn password_handler(
    _: HttpRequest,
    username: Username,
    password: Password,
) -> HandlerReturn {
    let repo = get_users_repository()
        .await
        .map_err(|_| Oauth2ErrorType::ServerError)?;

    let user = repo
        .find_by_username(username)
        .await
        .map_err(|_| Oauth2ErrorType::InvalidGrant)?;

    match user {
        Some(user) if repo.verify_password(&password, &user.password_hash) => {
            create_token_response(user.ext_id.unwrap())
                .await
                .map_err(|err| Oauth2ErrorType::InternalError(err.to_string()))
        }
        _ => Err(Oauth2ErrorType::InvalidGrant),
    }
}
