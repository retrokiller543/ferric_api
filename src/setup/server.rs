/// A macro for creating the actix web server. this can be used in a `main` function or in tests to
/// set up the basic configuration of the server, it does not start the server!
#[macro_export]
macro_rules! server {
    () => {{
        ::sqlx_utils::pool::initialize_db_pool($crate::setup::database::db_pool().await?);

        ::actix_web::HttpServer::new(move || {
            let cors = $crate::config::cors();
            let error_handler = actix_web::middleware::ErrorHandlers::new()
                .default_handler($crate::error::default_error_handler);

            $crate::app!(configure: [$crate::setup::server::configure_resources, $crate::setup::server::configure_state, $crate::setup::server::configure_index_service]; wrap: [error_handler, cors];)
        })
    }};
}

macro_rules! config {
    ($vis:vis $ident:ident($var:pat_param) $blk:block) => {
        ::paste::paste! {
            $vis fn [<configure_ $ident>]($var: &mut ServiceConfig) $blk
        }
    };
}

macro_rules! block_on_fut {
    (! match $expr:expr) => {
        match ::futures::executor::block_on($expr) {
            Ok(val) => val,
            Err(error) => {
                ::tracing::error!(
                    "Error occured when executing `{}`, {error}",
                    stringify!($expr)
                );
                panic!("Failed to run future: `{}`", stringify!($expr))
            }
        }
    };

    (match $expr:expr; $blk:block) => {
        match ::futures::executor::block_on($expr) {
            Ok(val) => val,
            Err(error) => $blk,
        }
    };

    ($expr:expr) => {
        ::futures::executor::block_on($expr)
    };
}

config! {
    pub state(cfg) {
        let state = block_on_fut!(!match app_state());
        let token_repo = *OAUTH_TOKEN_REPOSITORY;
        let client_repo = *OAUTH_CLIENTS_REPOSITORY;
        let user_repo = *USERS_REPOSITORY;

        cfg.app_data(state)
           .app_data(web::Data::new(token_repo))
           .app_data(web::Data::new(client_repo))
           .app_data(web::Data::new(user_repo));
    }
}

config! {
    pub resources(cfg) {
        EXTERNAL_RESOURCES.iter().for_each(|(name, url)| {
            cfg.external_resource(name, url);
        })
    }
}

config! {
    pub index_service(cfg) {
        match index_scope() {
            Ok(scope) => {cfg.service(scope);},
            Err(error) => {
                error!("failed to create the index scope with error {error}");
            }
        };
    }
}

use crate::endpoints::index_scope;
use crate::repositories::oauth_clients::OAUTH_CLIENTS_REPOSITORY;
use crate::repositories::oauth_token::OAUTH_TOKEN_REPOSITORY;
use crate::repositories::users::USERS_REPOSITORY;
use crate::state::app_state;
use crate::statics::EXTERNAL_RESOURCES;
use actix_web::web;
use actix_web::web::ServiceConfig;
use tracing::error;
