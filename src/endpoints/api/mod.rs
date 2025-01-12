use crate::openapi::ApiDocs;
use crate::statics::BASE_URL;
use crate::ServerResult;
use actix_web::{web, HttpResponse, Responder};
use tracing::warn;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as OtherServable};
use utoipa_swagger_ui::{Config, SwaggerUi};

pub(crate) mod v1;

/// All API endpoints
#[inline]
pub(crate) async fn api() -> ServerResult<impl actix_web::dev::HttpServiceFactory> {
    Ok(web::scope("/api")
        .service(v1::v1_service().await?)
        .service(docs()))
}

/// Only real reason we have this is to be able to put scoped middlewares for the docs, for example we can add auth middleware to secure the docs
#[inline]
fn docs() -> impl actix_web::dev::HttpServiceFactory {
    let openapi = ApiDocs::openapi();
    let config = Config::from("/api/docs/openapi.json");

    web::scope("/docs")
        .service(Redoc::with_url("/redoc", openapi.clone()))
        .service(
            SwaggerUi::new("/swagger/{_:.*}")
                .url("/openapi.json", openapi.clone())
                .config(config),
        )
        .service(RapiDoc::new("/api/docs/openapi.json").path("/rapidoc"))
        .service(Scalar::with_url("/scalar", openapi.clone()))
        .service(v1::v1_docs())
        .default_service(web::to(docs_index))
}

async fn docs_index() -> impl Responder {
    warn!("Documentation path not found, redirecting to default docs");

    let url = format!("{}/api/docs/scalar", *BASE_URL);

    HttpResponse::NotFound()
        .append_header(("Location", url))
        .finish()
}
