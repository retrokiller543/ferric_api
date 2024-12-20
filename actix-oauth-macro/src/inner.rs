use quote::quote;
use syn::{parse_macro_input, ItemFn, ReturnType};

pub(crate) fn oauth_function_inner(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    if input.sig.asyncness.is_none() {
        return syn::Error::new_spanned(input.sig.fn_token, "async fn is required")
            .to_compile_error()
            .into();
    }

    let mut signature = input.sig.clone();
    signature.asyncness = None;
    signature.output = ReturnType::Default;

    let content = input.block;
    let vis = input.vis;
    let attrs = input.attrs;

    let expanded = quote! {
        #(#attrs)*
        #vis #signature -> ::actix_oauth::handler::HandlerFuture {
            Box::pin(async #content)
        }
    };

    proc_macro::TokenStream::from(expanded)
}
