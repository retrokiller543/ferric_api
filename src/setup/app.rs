/// Basic macro for configuring the app, the configuration will be the
/// same for tests as for regular usage as this is the main way to run the app
macro_rules! app {
    ($state:ident, $cors:ident) => {
        actix_web::App::new()
            .app_data($state.clone())
            .service(crate::endpoints::index_scope())
            .wrap($cors)
    };
}

pub(crate) use app;
