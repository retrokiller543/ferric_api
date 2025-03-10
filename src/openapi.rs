use crate::dto::ErrorDTO;
use crate::endpoints::{api::v1::V1API, health::__path_health};
use crate::repositories::DatabaseHealth;
use crate::services::health::{ProcessStats, ServerHealth};
use std::collections::BTreeMap;
use utoipa::openapi::OpenApi as OpenApiSpec;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

/// Constructs a new struct that implements [`Modify`] trait for [`utoipa`] documentation.
///
/// This is a not ideal way to do it, but this is the best solution I came up with.
macro_rules! version_prefix {
    ($version:ident) => {
        ::paste::paste! {
            #[allow(dead_code)]
            #[doc(hidden)]
            pub(crate) struct [<Add $version:camel Prefix>];

            impl ::utoipa::Modify for [<Add $version:camel Prefix>] {
                fn modify(&self, openapi: &mut ::utoipa::openapi::OpenApi) {
                    let paths = openapi.paths.paths.clone();
                    let mut new_paths = ::std::collections::BTreeMap::new();

                    paths.iter().for_each(|(path, item)| {
                        let new_path = &format!("/api/{}{}", stringify!([<$version:snake>]), path);
                        new_paths.insert(new_path.clone(), item.clone());
                    });

                    openapi.paths.paths = new_paths;
                }
            }
        }
    };
}
pub(crate) use version_prefix;

version_prefix! {
    V1
}

pub(crate) struct NormalizePath;

impl Modify for NormalizePath {
    fn modify(&self, openapi: &mut OpenApiSpec) {
        let paths = openapi.paths.paths.clone();
        let mut new_paths = BTreeMap::new();

        paths.iter().for_each(|(path, item)| {
            let new_path = &path.replace("//", "/");

            new_paths.insert(new_path.clone(), item.clone());
        });

        openapi.paths.paths = new_paths;
    }
}

pub struct OpenApiSecurityConfig;

impl Modify for OpenApiSecurityConfig {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let bearer = SecurityScheme::Http(
            HttpBuilder::new()
                .scheme(HttpAuthScheme::Bearer)
                .description(Some("Bearer auth"))
                .build(),
        );

        if let Some(components) = &mut openapi.components {
            components.add_security_scheme("bearer_token", bearer);
        } else {
            openapi.components = Some(
                utoipa::openapi::ComponentsBuilder::new()
                    .security_scheme("bearer_token", bearer)
                    .build(),
            );
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    servers(
        (url = "http://localhost:{port}", description = "A local instance of the API", variables(("port" = (default = "8000", enum_values("8000"), description = "The port the API is running on. This port may change based on the PORT env variable when running the server")))),
    ),
    paths(health),
    nest(
        (path = "/", api = V1API),
    ),
    components(schemas(ErrorDTO, ServerHealth, DatabaseHealth), responses(ErrorDTO, ServerHealth, DatabaseHealth, ProcessStats)),
    tags(),
    modifiers(&NormalizePath, &OpenApiSecurityConfig)
)]
pub struct ApiDocs;
