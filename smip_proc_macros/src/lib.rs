mod service;
mod methods;

use darling::{Error, FromMeta};
use darling::ast::NestedMeta;
use service::ServiceArgs;
use proc_macro::TokenStream;
use syn::ItemStruct;
use quote::quote;

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

    let struct_name = struct_def.ident.clone();

    let id = args.id;
    let major_version = args.major_version;
    let minor_version = args.minor_version;
    
    quote!(
        #struct_def 

        #[automatically_derived]
        impl ::smip::ServiceDefinition for #struct_name {
            fn id(&self) -> ::smip::ServiceId {
                #id
            }
            fn major_version(&self) -> ::smip::MajorVersion {
                #major_version
            }
            fn minor_version(&self) -> ::smip::MinorVersion {
                #minor_version
            }
        }
    ).into()
}

#[proc_macro_attribute]
pub fn methods_impl(_args: TokenStream, item: TokenStream) -> TokenStream {
    let impl_block = syn::parse_macro_input!(item as syn::ItemImpl);
    methods::expand_methods_impl(impl_block);
    todo!()
}