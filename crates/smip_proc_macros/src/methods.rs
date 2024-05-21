use proc_macro2::TokenStream;
use someip_types::MethodId;
use std::collections::HashMap;
use syn::{parse_quote, spanned::Spanned, ImplItemFn, Meta, ReturnType, Type};
use quote::quote;

pub fn expand_methods_impl(mut impl_block: syn::ItemImpl) -> syn::Result<TokenStream> {
    check_valid_impl(&impl_block)?;

    let mut methods = Vec::new();
    
    let mut method_id_to_name = HashMap::new();

    for item in &mut impl_block.items {
        match item {
            syn::ImplItem::Fn(method) => {
                if let Some(attr_ix) = extract_method_attr(method) {
                    check_valid_method(method)?;
                    
                    let attribute = method.attrs.remove(attr_ix);
                    method.attrs.push(parse_quote!(#[allow(unused)]));
                    
                    let method_id = extract_method_id(&attribute.meta)?;

                    if let Some(method_name) = method_id_to_name.get(&method_id) {
                        return Err(syn::Error::new(method.sig.ident.span(), format!("method id {} is already used by method {}", method_id, method_name)));
                    }

                    methods.push((&*method, method_id));

                    method_id_to_name.insert(method_id, method.sig.ident.to_string());

                } else {
                    continue;
                }
            }
            _ => {
                return Err(syn::Error::new(item.span(), "only methods are allowed in methods_impl"));
            }
        }
    }

    let ty = &*impl_block.self_ty;

    let derived_service_methods_impl = derive_service_methods(ty, &methods)?;

    let output = quote!(
        #impl_block
        
        #derived_service_methods_impl
    );

    Ok(output)
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

    // Check if the first argument of this method is self
    let first_arg = method.sig.inputs.first();
    if let Some(syn::FnArg::Receiver(_)) = first_arg {
        // First argument is self
    } else {
        return Err(syn::Error::new(method.sig.inputs.span(), "method should have self as the first argument"));
    }

    let n_args = method.sig.inputs.len();

    // method should have zero or one arguments other than self
    if !(n_args == 1 || n_args == 2)  {
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

fn derive_service_methods(service_name: &Type, methods: &[(&ImplItemFn, MethodId)] ) -> syn::Result<TokenStream> {
    let methods = methods.iter().map(|(method, method_id)| {
        let method_name = &method.sig.ident;
        let method_id = *method_id;
        let return_type = &method.sig.output;

        let is_getter = method.sig.inputs.len() == 1;
        let has_return_type = matches!(return_type, ReturnType::Type(_, _));

        let parse_request_payload = quote!(
            let payload = message.get_payload();
            let arg = ::smip::FromPayload::from_payload(payload.get_data())?;
        );

        let call_method = if is_getter {
                quote!( 
                    let output = service.#method_name(); 
                )
        } else {
                quote!( 
                    #parse_request_payload
                    let output = service.#method_name(arg); 
                )
        };

        
        let write_response_payload = quote!(
            let result_payload = ::smip::ToPayload::to_payload(&output)?;

            let mut response = ::smip::Message::response(message);
            response.set_payload(&::smip::Payload::with_data(&result_payload));

        );

        let return_message = if has_return_type {
            quote!(
                #write_response_payload
                Ok(Some(response))
            ) 
        } else {
            quote!(
                Ok(None)
            )
        };

        quote!(
            builder.add_method(#method_id, |service, message| {   
                #call_method
                #return_message
            });
        )
    });

    let mut stream = TokenStream::new();

    for method in methods {
        stream.extend(method);
    }

    Ok(
        quote!(
            impl ::smip::ServiceMethods for #service_name {
                fn register_methods(builder: &mut ::smip::MethodsBuilder<#service_name>) {
                    #stream
                }
            }
        )
    )
}