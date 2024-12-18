/// A macro for creating the actix web server. this can be used in a `main` function or in tests to
/// set up the basic configuration of the server, it does not start the server!
macro_rules! server {
    () => {{
        let state = crate::state::app_state().await?;

        actix_web::HttpServer::new(move || {
            let cors = crate::config::cors();

            crate::setup::app::app!(state, cors)
        })
    }};
}
pub(crate) use server;
