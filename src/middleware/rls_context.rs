use crate::models::user::User;
use crate::types::UserContext;
use crate::utils::middleware_macros::define_middleware;
use actix_oauth::types::AccessToken;
use actix_web::HttpMessage;
use actix_web::dev::ServiceRequest;
use tracing::warn;

define_middleware! {
    #[derive(Debug)]
    pub struct RlsContextMiddleware = "RlsContextMiddleware" {},

    pub struct RlsContextMiddlewareService;

    |service: RlsContextMiddlewareService<S>, req: ServiceRequest| async move {
        let user_context = if let Some(user) = req.extensions().get::<User>() {
            if let Some(ext_id) = user.ext_id {
                let token = req.extensions().get::<AccessToken>().cloned();

                if token.is_none() {
                    warn!("No token found in context");
                    return Err(actix_web::error::ErrorUnauthorized("Access token was not found"))
                }

                let token = token.unwrap();

                UserContext::Authenticated {
                    ext_id,
                    token
                }
            } else {
                UserContext::Anonymous
            }
        } else {
            UserContext::Anonymous
        };

        req.extensions_mut().insert(user_context);

        let res = service.service.call(req).await?;

        Ok(res)
    }
}
