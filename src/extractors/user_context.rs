use crate::extractor;
use crate::prelude::UserContext;
use actix_web::HttpMessage;
use futures_util::future::{Ready, ready};
use std::convert::Infallible;
use tracing::warn;

extractor! {
    UserContext => <Error = Infallible, Future = Ready<Result<Self, Self::Error>>>(req, _) {
        let context = req.extensions().get::<Self>().cloned();

        if let Some(context) = context {
            ready(Ok(context))
        } else {
            let default_context = Self::default();
            warn!(context = %default_context, "Unable to extract user context, defaulting to Default");
            ready(Ok(default_context))
        }
    }
}
