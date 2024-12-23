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

    if let ReturnType::Type(_, ref mut ty) = signature.output {
        *ty = syn::parse_quote! { ::actix_oauth::handler::OAuthFuture<#ty> };
    } else {
        signature.output = ReturnType::Type(
            syn::token::RArrow::default(),
            Box::new(syn::parse_quote! { ::actix_oauth::handler::OAuthFuture<()> }),
        );
    }

    let content = input.block;
    let vis = input.vis;
    let attrs = input.attrs;

    let expanded = quote! {
        #(#attrs)*
        #vis #signature {
            Box::pin(async #content)
        }
    };

    proc_macro::TokenStream::from(expanded)
}
