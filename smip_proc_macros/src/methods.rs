use proc_macro2::TokenStream;
use syn::{spanned::Spanned, ImplItemFn};
use quote::quote;

pub fn expand_methods_impl(impl_block: syn::ItemImpl) -> syn::Result<TokenStream> {
    check_valid_impl(&impl_block)?;

    
    for item in &impl_block.items {
        match item {
            syn::ImplItem::Fn(method) => {
                
                check_valid_method(method)?;

                let method_name = &method.sig.ident;
                let method_block = &method.block;
                let method_vis = &method.vis;
                let method_sig = &method.sig;
                let method_block = &method.block;
                let method_block = quote! {
                    #method_vis #method_sig {
                        #method_block
                    }
                };
                println!("method_name: {:?}", method_name);
                println!("method_block: {:?}", method_block);
            }
            _ => {
                return Err(syn::Error::new(item.span(), "only methods are allowed in methods_impl"));
            }
        }
    }
    todo!()
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