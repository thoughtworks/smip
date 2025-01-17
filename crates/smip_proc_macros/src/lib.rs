mod service;
mod methods;

use darling::{Error, FromMeta};
use darling::ast::NestedMeta;
use service::ServiceArgs;
use proc_macro::TokenStream;
use syn::ItemStruct;

#[proc_macro_attribute]
pub fn service(args: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(Error::from(e).write_errors()); }
    };
    
    let struct_def = syn::parse_macro_input!(item as ItemStruct);

    let args = match ServiceArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => { return TokenStream::from(e.write_errors()); }
    };

    service::expand_service_impl(&struct_def, args).into()
}

#[proc_macro_attribute]
pub fn methods_impl(_args: TokenStream, item: TokenStream) -> TokenStream {
    let impl_block = syn::parse_macro_input!(item as syn::ItemImpl);
    methods::expand_methods_impl(impl_block).unwrap_or_else(|e| TokenStream::from(e.into_compile_error()).into())
    .into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn ui() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/ui-fail/*.rs");
        t.pass("tests/*.rs");
    }
}