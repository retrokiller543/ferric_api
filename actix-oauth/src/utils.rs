use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;

/// Returns a random alphanumeric string of length `length`.
pub fn random_string(length: usize) -> String {
    Alphanumeric.sample_string(&mut thread_rng(), length)
}

#[macro_export]
macro_rules! oauth2_handler {
    ($(#[$ty_meta:meta])* $vis:vis fn $name:ident($parameter:pat_param => ($($param:ty),*)) -> $return_type:ty) => {
        $crate::oauth2_handler! {$(#[$ty_meta])* $vis fn $name($parameter => ($($param),*)) -> $return_type {std::future::ready(Err(Oauth2ErrorType::UnsupportedGrantType))}}
    };

    ($(#[$ty_meta:meta])* $vis:vis fn $name:ident($parameter:pat_param => ($($param:ty),*)) -> $return_type:ty $block:block) => {
        #[derive(Clone)]
        $vis struct $name;

        #[allow(unused_parens)]
        impl AsyncFnOnce<($($param),*)> for $name {
            type CallOnceFuture = std::future::Ready<$return_type>;
            type Output = $return_type;

            extern "rust-call" fn async_call_once(
                self,
                $parameter: ($($param),*),
            ) -> Self::CallOnceFuture $block
        }

        #[allow(unused_parens)]
        impl AsyncFnMut<($($param),*)> for $name {
            type CallRefFuture<'a>
                = std::future::Ready<$return_type>
            where
                Self: 'a;

            extern "rust-call" fn async_call_mut(
                &mut self,
                $parameter: ($($param),*),
            ) -> Self::CallRefFuture<'_> $block
        }

        #[allow(unused_parens)]
        impl AsyncFn<($($param),*)> for $name {
            extern "rust-call" fn async_call(
                &self,
                $parameter: ($($param),*),
            ) -> Self::CallRefFuture<'_> $block
        }
    };
}
