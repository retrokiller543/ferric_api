extern crate proc_macro;
mod inner;

use proc_macro::TokenStream;

/// Example of user-defined [procedural macro attribute][1].
///
/// [1]: https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros
#[proc_macro_attribute]
pub fn oauth(args: TokenStream, input: TokenStream) -> TokenStream {
    inner::oauth_function_inner(args, input)
}
