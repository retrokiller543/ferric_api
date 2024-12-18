use actix_cors::Cors;

/// Default CORS config, for simplicity this is set to be permissive with credentials and any header allowed
#[inline(always)]
pub fn cors() -> Cors {
    Cors::permissive().supports_credentials().allow_any_header()
}
