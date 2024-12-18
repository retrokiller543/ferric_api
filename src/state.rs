use crate::ServerResult;
use actix_web::web;

/// Any potential state that might for the server, it is recommended to have more than one state
/// registered to not make this struct overly large and complex
#[derive(Clone)]
pub(crate) struct AppState;

/// Initializes the main state of the server
#[inline]
pub(crate) async fn app_state() -> ServerResult<web::Data<AppState>> {
    Ok(web::Data::new(AppState))
}
