use proc_macro2::TokenStream;
use someip_types::MethodId;
use syn::{parse_quote, spanned::Spanned, ImplItemFn, Meta};
use quote::quote;

pub fn expand_methods_impl(mut impl_block: syn::ItemImpl) -> syn::Result<TokenStream> {
    check_valid_impl(&impl_block)?;

    let mut methods = Vec::new();
    
    for item in &mut impl_block.items {
        match item {
            syn::ImplItem::Fn(method) => {
                check_valid_method(method)?;
                if let Some(attr_ix) = extract_method_attr(method) {
                    let attribute = method.attrs.remove(attr_ix);
                    method.attrs.push(parse_quote!(#[allow(unused)]));

                    let method_id = extract_method_id(&attribute.meta)?;
                    methods.push((method, method_id));

                } else {
                    continue;
                }
            }
            _ => {
                return Err(syn::Error::new(item.span(), "only methods are allowed in methods_impl"));
            }
        }
    }

    // let derived_service_methods_impl = derive_service_methods(&impl_block, &methods)?;

    Ok(quote!(
        #impl_block
        // derived_service_methods_impl
    ))
}

fn check_valid_impl(impl_block: &syn::ItemImpl) -> syn::Result<()> {
    // block should not be a trait impl
    if impl_block.trait_.is_some() {
        return Err(syn::Error::new(impl_block.span(), "trait impls are not supported are not supported in impl block"));
    }

    // block should not have generics
    if impl_block.generics.params.len() > 0 {
        return Err(syn::Error::new(impl_block.generics.span(), "generics are not supported in impl block"));
    }

    // block should not have a where clause
    if impl_block.generics.where_clause.is_some() {
        return Err(syn::Error::new(impl_block.generics.where_clause.span(), "where clauses are not supported in impl block"));
    }

    Ok(())
}

fn check_valid_method(method: &ImplItemFn) -> syn::Result<()> {
    // method should not have generics
    if method.sig.generics.params.len() > 0 {
        return Err(syn::Error::new(method.sig.generics.span(), "generics in method are not supported"));
    }

    // method should not have a where clause
    if method.sig.generics.where_clause.is_some() {
        return Err(syn::Error::new(method.sig.generics.where_clause.span(), "where clauses in method are not supported"));
    }

    // method should have exactly one argument
    if method.sig.inputs.len() != 2 {
        return Err(syn::Error::new(method.sig.inputs.span(), "method should have exactly one arguments"));
    }

    Ok(())
}

fn extract_method_attr(method: &ImplItemFn) -> Option<usize> {
    for (ix, attr) in method.attrs.iter().enumerate() {
        let meta = &attr.meta;
        let path = meta.path();

        if path.is_ident("smip_method") {
            return Some(ix);
        }
    }

    None
}
fn extract_method_id(meta: &Meta) -> syn::Result<MethodId> {
    match meta {
        Meta::List(meta) => {
            let meta: Meta = meta.parse_args()?;
            match meta {
                Meta::NameValue(meta) => {
                    meta.path.get_ident().map_or(Err(syn::Error::new(meta.span(), "expected id")), |ident| {
                        let id = ident.to_string();
                        if id != "id" {
                            return Err(syn::Error::new(meta.span(), "expected id"));
                        }
                        
                        match &meta.value {
                            syn::Expr::Lit(syn::ExprLit{lit, ..}) => {
                                match lit {
                                    syn::Lit::Int(int) => {
                                        let id = int.base10_parse::<u16>()?;
                                        Ok(id)
                                    },
                                    _ => Err(syn::Error::new(meta.span(), "method id should be a number"))
                                }
                            },
                            _ => return Err(syn::Error::new(meta.span(), "method id should be a number"))
                        }
                    })
                },
                _ => Err(syn::Error::new(meta.span(), "expected #[smip_method(id = xyzw)] to set method id"))
            }
        },
        _ => Err(syn::Error::new(meta.span(), "expected #[smip_method(id = xyzw)] to set method id"))
    }
}

fn derive_service_methods(impl_block: &syn::ItemImpl, methods: &[(&ImplItemFn, MethodId)] ) -> syn::Result<TokenStream> {
    todo!()
}