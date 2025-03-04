use crate::ServerResult;
use actix_web::web;

/// Any potential state that might for the server, it is recommended to have more than one state
/// registered to not make this struct overly large and complex
#[derive(Clone)]
pub struct AppState {
    started_at: chrono::DateTime<chrono::Utc>,
}

impl AppState {
    pub fn new() -> Self {
        let started_at = chrono::Utc::now();

        AppState { started_at }
    }

    pub fn started_at(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.started_at
    }
}

/// Initializes the main state of the server
#[inline]
pub async fn app_state() -> ServerResult<web::Data<AppState>> {
    Ok(web::Data::new(AppState::new()))
}
