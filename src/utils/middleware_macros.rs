/// Macro to define a middleware, removing most of the boilerplate code.
///
/// # Example
/// ```rust
/// # use ferric_api::define_middleware;
/// # use actix_web::dev::ServiceRequest;
/// define_middleware! {
///     #[derive(Debug)]
///     pub struct MyMiddleware {
///         config: String,
///     },
///     
///     pub struct MyMiddlewareService;
///     
///     |service: MyMiddlewareService<S>, req: ServiceRequest| async move {
///         // Your middleware logic here
///         service.service.call(req).await
///     }
/// }
/// ```
#[macro_export]
macro_rules! define_middleware {
    (
        $(#[$meta:meta])*
        $vis:vis struct $middleware_name:ident $(<$type_lifetime:lifetime>)? $(= $str_name:literal)? {
            $(
                $(#[$field_meta:meta])*
                $middleware_field:ident: $middleware_type:ty $(=> $field_method:ident)?
            ),* $(,)?
        },

        $(#[$service_meta:meta])*
        $service_vis:vis struct $service_name:ident;

        $call_fn:expr
    ) => {
        $(#[$meta])*
        $vis struct $middleware_name$(<$type_lifetime>)? {
            $(
                $(#[$field_meta])*
                pub $middleware_field: $middleware_type
            ),*
        }

        impl $middleware_name {
            /// Creates a new instance of the middleware
            #[inline]
            pub fn new($($middleware_field: $middleware_type),*) -> Self {
                Self {
                    $($middleware_field),*
                }
            }
        }

        impl<S, B> actix_web::dev::Transform<S, actix_web::dev::ServiceRequest> for $middleware_name
        where
            S: actix_web::dev::Service<
                actix_web::dev::ServiceRequest,
                Response = actix_web::dev::ServiceResponse<B>,
                Error = actix_web::Error
            > + 'static,
            B: 'static,
        {
            type Response = S::Response;
            type Error = S::Error;
            type Transform = $service_name<S>;
            type InitError = ();
            type Future = futures::future::Ready<Result<Self::Transform, Self::InitError>>;

            #[inline]
            fn new_transform(&self, service: S) -> Self::Future {
                futures::future::ok($service_name {
                    service: std::sync::Arc::new(service),
                    $($middleware_field: self.$middleware_field $(.$field_method())?),*
                })
            }
        }

        $(#[$service_meta])*
        $service_vis struct $service_name<S> {
            service: std::sync::Arc<S>,
            $($middleware_field: $middleware_type),*
        }

        impl<S, B> actix_web::dev::Service<actix_web::dev::ServiceRequest> for $service_name<S>
        where
            S: actix_web::dev::Service<
                actix_web::dev::ServiceRequest,
                Response = actix_web::dev::ServiceResponse<B>,
                Error = actix_web::Error
            > + 'static,
            B: 'static,
        {
            type Response = S::Response;
            type Error = S::Error;
            type Future = futures::future::LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

            #[inline]
            fn poll_ready(
                &self,
                cx: &mut std::task::Context<'_>
            ) -> std::task::Poll<Result<(), Self::Error>> {
                self.service.poll_ready(cx)
            }

            #[::tracing::instrument(skip_all, level = "debug" $(, name = $str_name)?)]
            #[inline]
            fn call(&self, req: actix_web::dev::ServiceRequest) -> Self::Future {
                let service = self.clone();
                Box::pin(async move {
                    ($call_fn)(service, req).await
                })
            }
        }

        impl<S> Clone for $service_name<S> {
            fn clone(&self) -> Self {
                Self {
                    service: self.service.clone(),
                    $($middleware_field: self.$middleware_field $(.$field_method())?),*
                }
            }
        }
    };
}

pub(crate) use define_middleware;
