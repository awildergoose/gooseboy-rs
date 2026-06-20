//! Used for helper macros to export Gooseboy host functions.

use proc_macro::TokenStream;
use quote::{ToTokens, format_ident, quote};
use syn::{FnArg, ItemFn, PatType, spanned::Spanned};

/// Creates a wrapper for a Gooseboy host function.
///
/// # Panics
///
/// Panics if an argument really fucks everything up for us!
#[allow(clippy::too_many_lines)]
fn make_wrapper(item: &TokenStream, export_name: &str, call_args_vec: &[FnArg]) -> TokenStream {
    let mut input_fn: ItemFn = match syn::parse::<ItemFn>(item.clone()) {
        Ok(i) => i,
        Err(e) => return TokenStream::from(e.to_compile_error()),
    };

    if !input_fn.sig.generics.params.is_empty() || input_fn.sig.asyncness.is_some() {
        return TokenStream::from(quote! {
            compile_error!("export does not support generics or async functions");
        });
    }
    if input_fn
        .sig
        .inputs
        .iter()
        .any(|arg| matches!(arg, FnArg::Receiver(_)))
    {
        return TokenStream::from(quote! {
            compile_error!("export does not support methods with `self`");
        });
    }

    let orig_ident = input_fn.sig.ident.clone();
    let export_ident = format_ident!("{}", export_name);

    let internal_ident = if orig_ident == export_ident {
        format_ident!("__gooseboy_internal_{}", orig_ident)
    } else {
        orig_ident
    };

    input_fn.sig.ident = internal_ident.clone();

    let mut attrs = input_fn.attrs.clone();
    let hidden_attr: syn::Attribute = syn::parse_quote!(#[doc(hidden)]);
    // Clippy for some reason just does not give a fuck about this?
    let allow_clippy: syn::Attribute = syn::parse_quote!(#[allow(clippy::used_underscore_binding)]);
    attrs.insert(0, hidden_attr);
    attrs.insert(1, allow_clippy);
    input_fn.attrs = attrs;

    let original_args = input_fn.sig.inputs.clone();
    input_fn.sig.inputs.clear();

    for (i, arg) in call_args_vec.iter().enumerate() {
        if let Some(arg) = original_args.get(i) {
            input_fn.sig.inputs.insert(i, arg.clone());
        } else {
            input_fn.sig.inputs.insert(i, arg.clone());
        }
    }

    {
        let left = call_args_vec.iter().map(|u| match u {
            FnArg::Typed(pat_type) => {
                let ty = &*pat_type.ty;
                ty.to_token_stream().to_string()
            }
            FnArg::Receiver(_) => String::new(),
        });
        let right = input_fn.sig.inputs.iter().map(|u| match u {
            FnArg::Typed(pat_type) => {
                let ty = &*pat_type.ty;
                ty.to_token_stream().to_string()
            }
            FnArg::Receiver(_) => String::new(),
        });

        if !left.clone().eq(right.clone()) {
            return syn::Error::new(
                input_fn.span(),
                format!(
                    "The function argument types do not match. (expected {}, got {})",
                    left.map(|u| u.to_token_stream().to_string())
                        .collect::<Vec<String>>()
                        .first()
                        .unwrap(),
                    right
                        .map(|u| u.to_token_stream().to_string())
                        .collect::<Vec<String>>()
                        .first()
                        .unwrap(),
                ),
            )
            .to_compile_error()
            .into();
        }
    }

    let call_args_vec: Vec<proc_macro2::TokenStream> = input_fn
        .sig
        .inputs
        .iter()
        .map(|arg| match arg {
            FnArg::Typed(pat_type) => {
                let pat = &*pat_type.pat;
                quote! { #pat }
            }
            FnArg::Receiver(_) => quote! {},
        })
        .collect();

    let wrapper_inputs = &input_fn.sig.inputs;
    let goose_internal_ident = format_ident!("__internal_{}", export_name);

    let call_args_for_goose = call_args_vec.clone();
    let call_args_for_user = call_args_vec;

    let expanded = quote! {
        #input_fn

        #[unsafe(no_mangle)]
        pub extern "C" fn #export_ident(#wrapper_inputs) {
            gooseboy::#goose_internal_ident(#(#call_args_for_goose),*);
            let res = std::panic::catch_unwind(|| {
                #internal_ident(#(#call_args_for_user),*);
            });

            if res.is_err() {
                gooseboy::__internal_caught_unwind(res);
            }
        }
    };

    // for debugging :)
    // syn::Error::new(expanded.span(), expanded.to_string())
    //     .to_compile_error()
    //     .into()
    TokenStream::from(expanded)
}

/// This is called on startup.
#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    make_wrapper(&item, "main", &[])
}

/// This is called every frame.
///
/// # Panics
///
/// Panics if something has gone REALLY wrong!
#[proc_macro_attribute]
pub fn update(_attr: TokenStream, item: TokenStream) -> TokenStream {
    make_wrapper(
        &item,
        "update",
        &[FnArg::Typed(
            syn::parse::<PatType>(
                quote! {
                    nano_time: i64
                }
                .into(),
            )
            .as_ref()
            .unwrap()
            .clone(),
        )],
    )
}

/// This is called when the `GooseGPU` is ready to receive commands.
#[proc_macro_attribute]
pub fn gpu_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    make_wrapper(&item, "gpu_main", &[])
}
