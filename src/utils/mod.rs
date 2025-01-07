pub(crate) mod batch;

/// A macro for creating OpenAPI-documented API scopes in an actix-web application.
///
/// # Overview
/// This macro generates both the routing infrastructure and OpenAPI documentation for API endpoints.
/// It supports versioning, guards, services, and various OpenAPI documentation features through a
/// declarative syntax.
///
/// # Structure
/// The macro accepts two main variants:
/// 1. With version - includes version-specific routing and documentation
/// 2. Without version - creates standard routing and documentation
///
/// # Features
/// - API scope creation with configurable endpoints
/// - Optional version prefixing
/// - Guard integration
/// - Service registration
/// - Path registration
/// - Comprehensive OpenAPI documentation including:
///   - Extra paths
///   - Tags
///   - Schemas
///   - Responses
///   - Nested APIs
///   - Documentation modifiers
///
/// # Generated Components
/// The macro generates:
/// - An OpenAPI struct with documentation
/// - Service factory functions for routing
/// - When versioned, additional documentation endpoints for Swagger UI, ReDoc, and RapiDoc
///
/// # Examples
/// Basic usage without versioning:
/// ```rust
/// api_scope! {
///     pub my_api = "/api";
///     
///     services: [health_check, status];
///     paths: [user_routes, admin_routes];
///     
///     docs: {
///         tags: ["Users", "Admin"];
///         schemas: [User, Admin];
///         responses: [ErrorResponse];
///     }
/// }
/// ```
///
/// Usage with versioning:
/// ```rust
/// api_scope! {
///     pub v1_api = "/v1";
///     
///     version: V1;
///     
///     guard: Get;
///     services: [auth_service, user_service];
///     
///     docs: {
///         tags: ["Auth", "Users"];
///         schemas: [AuthRequest, UserProfile];
///         nested: [("/external", external_api::ExternalAPI)];
///         modifiers: [SecurityAddon];
///     }
/// }
/// ```
///
/// # Internal Working Details
///
/// ## Version-specific Generation
/// When a version is specified, the macro:
/// 1. Generates a version prefix using the `version_prefix!` macro
/// 2. Creates a documentation service function that sets up:
///    - ReDoc UI at `/redoc`
///    - Swagger UI at `/swagger/*`
///    - RapiDoc at `/rapidoc`
///    - Scalar at `/scalar`
/// 3. Applies version-specific prefixes to all routes
///
/// ## OpenAPI Structure Generation
/// The macro generates a struct named `{ident}API` that implements `utoipa::OpenApi`:
/// - Includes all specified paths, schemas, and responses
/// - Incorporates nested APIs if specified
/// - Applies any provided modifiers
/// - Generates documentation based on provided tags
///
/// ## Service Factory
/// Creates a function named `{ident}_service` that:
/// 1. Sets up an actix-web scope with the specified endpoint
/// 2. Applies any specified guards
/// 3. Registers all provided services and paths
///
/// # Technical Notes
///
/// - Uses the `paste!` macro for identifier manipulation
/// - Leverages utoipa's derive macros for OpenAPI specification generation
/// - Integrates with multiple UI providers (Swagger, ReDoc, RapiDoc, Scalar)
/// - Supports conditional guard application
/// - Handles optional parameters through macro repetition
///
/// # Dependencies
/// Requires the following crates:
/// - actix-web
/// - utoipa
/// - utoipa-swagger-ui
/// - utoipa-redoc
/// - utoipa-rapidoc
/// - utoipa-scalar
/// - paste
///
/// # Implementation Details
/// The macro uses multiple internal rules that handle different cases:
/// 1. Full version-aware API with documentation
/// 2. Standard API with documentation
/// 3. Basic API scope without documentation
///
/// Each rule processes its input differently while maintaining consistent
/// output structure and functionality.
macro_rules! api_scope {
    {
        $vis:vis $ident:ident = $endpoint:literal;

        version: $version_ident:ident;

        $(guard: $guard_ident:ident;)?
        $(services: [$($service:ident),* $(,)?] ;)?
        $(paths: [$($path:path),* $(,)?] ;)?

        docs: {
            $( extra_paths: [$($extra_path:path),* $(,)?] $(;)? )?
            $( tags: [$($tag:literal),* $(,)?] $(;)? )?
            $( schemas: [$($schema_path:path),* $(,)?] $(;)? )?
            $( responses: [$($responses_path:path),* $(,)?] $(;)? )?
            $( nested: [$(($nested_path:literal, $nested_api:path)),* $(,)?] $(;)? )?
            $( modifiers: [$($modifier:ident),* $(,)?] $(;)? )?
        }
    } => {
        ::paste::paste! {
            crate::openapi::version_prefix! {
                $version_ident
            }

            #[inline]
            pub(crate) fn [< $version_ident:snake _docs >]() -> impl ::actix_web::dev::HttpServiceFactory {
                use ::utoipa_redoc::Servable;
                use ::utoipa_scalar::Servable as OtherServable;
                let openapi = <[<$ident:camel API>] as ::utoipa::OpenApi>::openapi();
                let config = ::utoipa_swagger_ui::Config::from(concat!("/api/docs/", stringify!([<$version_ident:lower>]), "/openapi.json"));

                ::actix_web::web::scope(concat!("/", stringify!([<$version_ident:lower>])))
                    .service(::utoipa_redoc::Redoc::with_url("/redoc", openapi.clone()))
                    .service(
                        ::utoipa_swagger_ui::SwaggerUi::new("/swagger/{_:.*}")
                            .url("/openapi.json", openapi.clone())
                            .config(config),
                    )
                    .service(::utoipa_rapidoc::RapiDoc::new(concat!("/api/docs/", stringify!([<$version_ident:lower>]), "/openapi.json")).path("/rapidoc"))
                    .service(::utoipa_scalar::Scalar::with_url("/scalar", openapi.clone()))
            }

            crate::utils::api_scope! {$vis $ident = $endpoint;$(guard: $guard_ident;)?$( services: [$($service),*] ; )?$( paths: [$($path),*] ; )?
                docs: {
                    $(extra_paths: [$($extra_path),*];)?
                    $(tags: [$($tag),*];)?
                    $(schemas: [$($schema_path),*];)?
                    $(responses: [$($responses_path),*];)?
                    $(nested: [$(($nested_path, $nested_api)),*];)?
                    modifiers: [[<Add $version_ident:camel Prefix>] $($(, $modifier)*)?];
                }
            }
        }
    };

    {
        $vis:vis $ident:ident = $endpoint:literal;

        $(guard: $guard_ident:ident;)?
        $(services: [$($service:ident),* $(,)?];)?
        $(paths: [$($path:path),* $(,)?];)?

        docs: {
            $( extra_paths: [$($extra_path:path),* $(,)?] $(;)? )?
            $( tags: [$($tag:literal),* $(,)?] $(;)? )?
            $( schemas: [$($schema_path:path),* $(,)?] $(;)? )?
            $( responses: [$($responses_path:path),* $(,)?] $(;)? )?
            $( nested: [$(($nested_path:literal, $nested_api:path)),* $(,)?] $(;)? )?
            $( modifiers: [$($modifier:ident),* $(,)?] $(;)? )?
        }
    } => {
        ::paste::paste! {
            #[derive(::utoipa::OpenApi)]
            #[openapi(
                $(
                    nest($((path = $nested_path, api = $nested_api)),*),
                )?
                paths(
                    $(
                        $($path),*
                    )?
                    $(, $($extra_path),*)?
                ),
                components(
                    $(
                        schemas($($schema_path),*),
                    )?
                    $(
                        responses($($responses_path),*)
                    )?
                ),
                $( tags($($tag),*), )?
                modifiers($($(&$modifier),*)?)
            )]
            $vis struct [<$ident:camel API>];
        }

        crate::utils::api_scope! {$vis $ident = $endpoint;$(guard: $guard_ident;)?$( services: [$($service),*] ; )?$( paths: [$($path),*] ; )?}
    };

    {
        $vis:vis $ident:ident = $endpoint:literal;

        $(guard: $guard_ident:ident;)?
        $( services: [$($service:ident),* $(,)?] ; )?
        $( paths: [$($path:path),* $(,)?] ; )?
    } => {
        ::paste::paste! {
            #[inline]
            $vis fn [<$ident:snake _service>]() -> impl ::actix_web::dev::HttpServiceFactory {
                ::actix_web::web::scope($endpoint)
                $(
                    .guard(::actix_web::guard::$guard_ident())
                )?
                $(
                    $(
                        .service($path)
                    )*
                )?
                $(
                    $(
                        .service($service())
                    )*
                )?
            }
        }
    };
}

pub(crate) use api_scope;
