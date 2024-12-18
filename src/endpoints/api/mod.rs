use crate::openapi::ApiDocs;
use actix_web::web;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as OtherServable};
use utoipa_swagger_ui::{Config, SwaggerUi};

pub(crate) mod v1;

/// All API endpoints
pub(crate) fn api() -> impl actix_web::dev::HttpServiceFactory {
    web::scope("/api")
        .service(v1::v1_endpoints())
        .service(docs())
}

/// Only real reason we have this is to be able to put scoped middlewares for the docs, for example we can add auth middleware to secure the docs
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
}
