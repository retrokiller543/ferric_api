use crate::ApiResult;
use actix_web::{HttpResponse, Responder};

pub(crate) async fn check_health() -> ApiResult<impl Responder> {
    Ok(HttpResponse::Ok())
}
