use crate::dto::*;
use crate::openapi::{AddV1Prefix, NormalizePath};
use actix_web::web;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as OtherServable};
use utoipa_swagger_ui::{Config, SwaggerUi};

#[derive(OpenApi)]
#[openapi(
    paths(),
    nest(),
    components(schemas(Error), responses(Error)),
    tags(),
    modifiers(&AddV1Prefix, &NormalizePath)
)]
pub struct DocsV1;

/// All v1 API endpoints
#[inline]
pub(crate) fn v1_endpoints() -> impl actix_web::dev::HttpServiceFactory {
    web::scope("/v1")
}

/// Documentation for only the v1 API. This does not include the docs for non `/api/v1` endpoints as that is done in `docs`
#[inline]
pub(crate) fn v1_docs() -> impl actix_web::dev::HttpServiceFactory {
    let openapi = DocsV1::openapi();
    let config = Config::from("/api/docs/v1/openapi.json");

    web::scope("/v1")
        .service(Redoc::with_url("/redoc", openapi.clone()))
        .service(
            SwaggerUi::new("/swagger/{_:.*}")
                .url("/openapi.json", openapi.clone())
                .config(config),
        )
        .service(RapiDoc::new("/api/docs/v1/openapi.json").path("/rapidoc"))
        .service(Scalar::with_url("/scalar", openapi.clone()))
}
