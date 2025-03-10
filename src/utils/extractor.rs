/// Creates a custom Actix extractor with minimal boilerplate.
///
/// This macro simplifies the implementation of Actix's [`FromRequest`](actix_web::FromRequest) trait by generating
/// the necessary trait implementation with configurable error and future types.
///
/// # Syntax Variants
///
/// The macro supports three syntax variants with increasing levels of customization:
///
/// ## Basic Variant (No Parameters)
/// ```no_compile
/// extractor!(MyExtractor => <Error = MyError, Future = MyFuture>() {
///     // Implementation block
/// });
/// ```
/// This variant creates an extractor where both the request and payload parameters
/// are not explicitly named. The request parameter is ignored, and the payload
/// parameter can be accessed with the `*` pattern.
///
/// ## Request Parameter Only
/// ```no_compile
/// extractor!(MyExtractor => <Error = MyError, Future = MyFuture>(req) {
///     // Implementation block where `req` is available
/// });
/// ```
/// This variant allows you to name and use the request parameter, while the
/// payload parameter is still accessible via the `*` pattern.
///
/// ## Full Customization
/// ```no_compile
/// use ferric_api::extractor;
///
/// extractor!(MyExtractor => <Error = MyError, Future = MyFuture>(req, payload) {
///     // Implementation block where both `req` and `payload` are available
/// });
/// ```
/// This variant gives full control, allowing you to name and use both the
/// request and payload parameters.
///
/// # Parameters
///
/// - `$struct_ident`: The name of your extractor struct
/// - `$error_ty`: The name of the error type parameter (typically `Error`)
/// - `$error`: The actual error type for the extractor
/// - `$future_ty`: The name of the future type parameter (typically `Future`)
/// - `$future`: The actual future type for the extractor
/// - `$req_ident`: Pattern to bind the HTTP request parameter (optional)
/// - `$payload_ident`: Pattern to bind the payload parameter (optional)
/// - `$block`: The implementation block for the `from_request` method
///
/// # Example
///
/// ```rust
/// use actix_web::{Error, HttpRequest};
/// use futures::future::{ready, Ready};
///
/// use ferric_api::extractor;
///
/// struct Token(String);
///
/// extractor!(Token => <Error = Error, Future = Ready<Result<Self, Self::Error>>>(req, _) {
///     let auth_header = req.headers().get("Authorization");
///
///     match auth_header {
///         Some(header) => {
///             if let Ok(auth_str) = header.to_str() {
///                 if auth_str.starts_with("Bearer ") {
///                     let token = auth_str[7..].to_string();
///                     return ready(Ok(Token(token)));
///                 }
///             }
///         }
///         None => {}
///     }
///
///     ready(Err(Error::from(std::io::Error::new(
///         std::io::ErrorKind::Other,
///         "No valid authorization token found",
///     ))))
/// });
/// ```
///
/// This example creates a `Token` extractor that extracts a bearer token from
/// the Authorization header.
#[macro_export]
macro_rules! extractor {
    ($struct_ident:ident => <$error_ty:ident = $error:ty, $future_ty:ident = $future:ty>($req_ident:pat_param, $payload_ident:pat_param) $block:block) => {
        impl ::actix_web::FromRequest for $struct_ident {
            type $error_ty = $error;
            type $future_ty = $future;

            fn from_request($req_ident: &::actix_web::HttpRequest, $payload_ident: &mut ::actix_web::dev::Payload) -> Self::Future $block
        }
    };
}
