/// A macro for creating the actix web server. this can be used in a `main` function or in tests to
/// set up the basic configuration of the server, it does not start the server!
macro_rules! server {
    () => {{
        let state = crate::state::app_state().await?;
        let oauth_client_repo = ::actix_web::web::Data::new(crate::repositories::oauth_clients::OAuthClientsRepository::new().await?);

        ::actix_web::HttpServer::new(move || {
            let cors = crate::config::cors();
            let error_handler = actix_web::middleware::ErrorHandlers::new()
                .default_handler(crate::error::default_error_handler);
            let index_scope = crate::endpoints::index_scope();


            crate::setup::app::app!(state: [state, oauth_client_repo]; service: [index_scope]; wrap: [error_handler, cors];)
        })
    }};
}
//crate::setup::app::app!(state, cors, error_handler)

pub(crate) use server;
