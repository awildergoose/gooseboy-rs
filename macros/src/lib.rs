use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{FnArg, ItemFn};

fn make_wrapper(item: &TokenStream, export_name: &str) -> TokenStream {
    let input_fn: ItemFn = match syn::parse::<ItemFn>(item.clone()) {
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

    let mut renamed_fn = input_fn.clone();
    renamed_fn.sig.ident = internal_ident.clone();

    let mut attrs = renamed_fn.attrs.clone();
    let hidden_attr: syn::Attribute = syn::parse_quote!(#[doc(hidden)]);
    let allow_clippy: syn::Attribute = syn::parse_quote!(#[allow(clippy::used_underscore_binding)]);
    attrs.insert(0, hidden_attr);
    attrs.insert(1, allow_clippy);
    renamed_fn.attrs = attrs;

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
        #renamed_fn

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

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    make_wrapper(&item, "main")
}

#[proc_macro_attribute]
pub fn update(_attr: TokenStream, item: TokenStream) -> TokenStream {
    make_wrapper(&item, "update")
}

#[proc_macro_attribute]
pub fn gpu_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    make_wrapper(&item, "gpu_main")
}
