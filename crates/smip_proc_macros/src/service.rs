use darling::FromMeta;
use proc_macro2::TokenStream;
use someip_types::{MajorVersion, MinorVersion};
use syn::ItemStruct;
use quote::quote;

#[derive(Debug, FromMeta)]
pub struct ServiceArgs {
    pub id: u16,
    pub major_version: Option<MajorVersion>,
    pub minor_version: Option<MinorVersion>
}

pub fn expand_service_impl(struct_def: &ItemStruct, args: ServiceArgs) -> TokenStream {
    let struct_name = struct_def.ident.clone();

    let id = args.id;
    let major_version = args.major_version.unwrap_or(MajorVersion::default());
    let minor_version = args.minor_version.unwrap_or(MinorVersion::default());

    quote!(
        #struct_def 

        #[automatically_derived]
        impl ::smip::ServiceDefinition for #struct_name {
            fn id() -> ::smip::ServiceId {
                #id
            }
            fn major_version() -> ::smip::MajorVersion {
                #major_version
            }
            fn minor_version() -> ::smip::MinorVersion {
                #minor_version
            }
        }
    )
}