pub mod authorization;
pub mod oauth_client;
pub mod oauth_error;
pub mod oauth_request;
pub mod token_response;

pub use authorization::*;
pub use oauth_client::*;
pub use oauth_error::*;
pub use oauth_request::*;
pub use token_response::*;

// TODO: Move this to a better place
#[macro_export]
macro_rules! impl_responder {
    ($ident:ident $(<$($lifetime:lifetime $(,)?)? $($generic:ident),*>)?) => {
        impl$(<$($lifetime,)? $($generic),*>)? ::actix_web::Responder for $ident$(<$($lifetime,)? $($generic),*>)? {
            type Body = ::actix_web::body::BoxBody;

            fn respond_to(self, _req: &::actix_web::HttpRequest) -> ::actix_web::HttpResponse<Self::Body> {
                ::actix_web::HttpResponse::Ok().json(&self)
            }
        }
    };
}
