/// Basic macro for configuring the app, the configuration will be the
/// same for tests as for regular usage as this is the main way to run the app
#[macro_export]
macro_rules! app {
    ($state:ident, $cors:ident, $error_handler:ident) => {
        actix_web::App::new()
            .app_data($state.clone())
            .wrap($error_handler)
            .service($crate::endpoints::index_scope())
            .wrap($cors)
    };

    (
        $(state: [$($state_ident:expr),* $(,)?];)?
        $(service: [$($service_ident:expr),* $(,)?];)?
        $(configure: [$($config:expr),* $(,)?];)?
        $(wrap: [$($wrap_ident:expr),* $(,)?];)?
    ) => {
        ::actix_web::App::new()
        $(
            $(.configure($config))*
        )?
        $(
            $(.app_data($state_ident.clone()))*
        )?
        $(
            $(.service($service_ident))*
        )?
        $(
            $(.wrap($wrap_ident))*
        )?
    };
}
