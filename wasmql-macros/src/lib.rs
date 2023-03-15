use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, Item, ItemTrait, TraitItem, Visibility, FnArg, Receiver, PatType};

macro_rules! error {
    ($tokens: expr, $message: expr) => {
        return syn::Error::new_spanned($tokens, $message).to_compile_error().into()
    };
}

#[proc_macro_attribute]
pub fn api(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let api = parse_macro_input!(input as ItemTrait);
    let input = api.clone();

    let name = api.ident;

    if !matches!(api.vis, Visibility::Public(_)) {
        error!(api.vis, "the trait must be public");
    }

    let mut funcs = vec![];

    for item in api.items {
        let TraitItem::Method(method) = item else {
            error!(item, "the trait must only contain methods");
        };

        if let Some(attr) = method.attrs.first() {
            error!(attr, "the method must not have any attributes");
        }

        if let Some(default) = method.default {
            error!(default, "the method must not have a default body");
        }

        let sig = method.sig;

        if sig.ident.to_string().starts_with("__") {
            error!(sig.ident, "the method identifier must not begin with a double underscore");
        }

        let mut inputs = sig.inputs.iter();

        if !matches!(
            inputs.next(), 
            Some(FnArg::Receiver(Receiver { attrs, reference: None, mutability: None, .. }))
            if attrs.is_empty()
        ) {
            error!(sig.inputs, "the method's first argument must be self");
        }

        let Some(FnArg::Typed(PatType { attrs, ty, .. })) = inputs.next() else {
            error!(sig.inputs, "the method must have a second argument.")
        };

        if let Some(attr) = method.attrs.first() {
            error!(attr, "the method's second argument must not have any attributes");
        }

        if let Some(arg) = inputs.next() {
            error!(arg, "the method must have no more than 2 arguments");
        }

        let rust_type = ty;

        let decoder = format_ident!("dec_{}", sig.ident);
        let encoder = format_ident!("enc_{}", sig.ident);

        funcs.push(quote! {
            #[no_mangle]
            unsafe fn #decoder (ptr: *mut u8, len: usize) -> ::wasmql::frontend::JsValue {
                ::wasmql::frontend::decode::<#rust_type>(ptr, len)
            }

            #[no_mangle]
            fn #encoder (val: ::wasmql::frontend::JsValue) -> ::wasmql::frontend::JsValue {
                ::wasmql::frontend::encode::<#rust_type>(val)
            }
        })
    }

    let tokens = quote! {
        #[cfg(target_arch = "wasm32")]
        #[allow(non_snake_case)]
        mod #name {
            use super::*;

            #(#funcs)*
        }

        #[cfg(not(target_arch = "wasm32"))]
        #input
    };

    tokens.into()
}

#[proc_macro_attribute]
pub fn data(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let data = parse_macro_input!(input as Item);

    let vis = match &data {
        Item::Enum(item) => &item.vis,
        Item::Struct(item) => &item.vis,
        Item::Union(item) => &item.vis,
        _ => error!(data, "the attribute must only be used on an enum, struct or union definition"),
    };

    if !matches!(vis, Visibility::Public(_)) {
        error!(vis, "the item must be public");
    }

    let tokens = quote! {
        #[derive(::wasmql::Deserialize, ::wasmql::Serialize)]
        #data
    };

    tokens.into()
}
