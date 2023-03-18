use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::punctuated::Punctuated;
use syn::token::Paren;
use syn::{parse_macro_input, Item, ItemTrait, TraitItem, Visibility, FnArg, Receiver, PatType, TypeTuple, Index};

macro_rules! error {
    ($tokens: expr, $message: expr) => {
        return syn::Error::new_spanned($tokens, $message).to_compile_error().into()
    };
}

/// Attribute for marking a rust trait as defining a WasmQL API.
/// 
/// This attribute can be applied to a rust trait that only contains
/// methods, whose signatures are `fn(self, T) -> U` where `T` and
/// `U` are enums or structs marked with the `#[data]` attribute.
/// 
/// This attribute has no effects to the underlying trait if compiled for
/// the backend, but will drastically change it if compiled to `wasm32`.
/// 
/// # Examples
/// 
/// ```no_run
/// #[wasmql::data]
/// pub struct Login {
///     username: String,
///     password: String,
/// }
/// 
/// #[wasmql::data]
/// pub struct Session(String);
/// 
/// #[wasmql::api]
/// pub trait MyApi {
///     fn login(self, login: Login) -> Session;
/// 
///     fn get_resource(self, session: Session) -> String;
/// }
/// ```
#[proc_macro_attribute]
pub fn api(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemTrait);

    let name = &input.ident;

    if !matches!(input.vis, Visibility::Public(_)) {
        error!(input.vis, "the trait must be public");
    }

    // Wasm funcs exported to js for encoding/decoding. 
    let mut extern_funcs = vec![];

    // Arms of the match expression in the backend dispatcher. 
    let mut dispatcher_arms = vec![];

    // For each item in this trait.
    for (i, item) in input.items.iter().enumerate() {
        // Is a method.
        let TraitItem::Fn(fun) = item else {
            error!(item, "the trait must only contain methods");
        };

        // Method has no attributes.
        if let Some(attr) = fun.attrs.first() {
            error!(attr, "the method must not have any attributes");
        }

        // Method has no body.
        if let Some(default) = &fun.default {
            error!(default, "the method must not have a default body");
        }

        let sig = &fun.sig;
        let ident = &sig.ident;

        // Name does not begin with "__".
        if ident.to_string().starts_with("__") {
            error!(ident.clone(), "the method identifier must not begin with a double underscore");
        }

        let mut inputs = sig.inputs.iter();

        // First argument must be `self`.
        let Some(FnArg::Receiver(Receiver { attrs, reference: None, mutability: None, .. })) = inputs.next() else {
            error!(&sig.inputs[0], "the method's first argument must be self");
        };

        // First argument must not have attributes.
        if let Some(attr) = attrs.first() {
            error!(attr, "the method's inputs must not have any attributes");
        }

        // Argument tuple (excluding self).
        let mut tuple = TypeTuple {
            paren_token: Paren::default(),
            elems: Punctuated::new(),
        };

        // For each subsequent arguments to this trait method.
        for arg in inputs {
            let FnArg::Typed(PatType { attrs, ty, .. }) = arg else { unreachable!() };

            // Argument has no attributes.
            if let Some(attr) = attrs.first() {
                error!(attr, "the method's arguments must not have any attributes");
            }

            tuple.elems.push(*ty.clone());
        }

        // Names of the decoder and encoder funcs.
        let decoder = format_ident!("dec_{}", ident);
        let encoder = format_ident!("enc_{}", ident);

        extern_funcs.push(quote! {
            #[no_mangle]
            unsafe fn #decoder(ptr: *mut u8, len: usize) -> ::wasmql::frontend::JsValue {
                ::wasmql::frontend::decode::<#tuple>(ptr, len)
            }

            #[no_mangle]
            fn #encoder(val: ::wasmql::frontend::JsValue) -> ::wasmql::frontend::JsValue {
                ::wasmql::frontend::encode::<#tuple>(val)
            }
        });

        let indexes = (0..tuple.elems.len()).map(Index::from);

        dispatcher_arms.push(quote! {
            #i => {
                let tuple = ::wasmql::backend::decode::<#tuple>(bytes)?;
                let value = self.#ident(#(tuple.#indexes),*);
                ::wasmql::backend::encode(&value)
            }
        });
    }

    let tokens = quote! {
        #[cfg(target_arch = "wasm32")]
        #[allow(non_snake_case)]
        mod #name {
            use super::*;

            #(#extern_funcs)*
        }

        #[cfg(not(target_arch = "wasm32"))]
        #input
    };

    tokens.into()
}

/// Attribute for marking a rust type as a WasmQL data-transfer object.
/// 
/// This attribute can be applied to an enum or struct definition.
/// It is simply an alias for deriving [`serde`](https://docs.rs/serde/latest/serde/)'s 
/// [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) and [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) traits,
/// without the need to add `serde` as an explicit dependency and importing the relevant
/// derive macros in scope.
/// 
/// A type marked as `data` can then be used as input or output for an API method, this is why
/// this type must be public.
/// 
/// # Examples
/// 
/// ```no_run
/// #[wasmql::data]
/// pub struct Person {
///     name: String,
///     age: i64,
/// }
/// 
/// #[wasmql::data]
/// pub enum User {
///     Unlogged,
///     Logged(Person),
/// }
/// ```
#[proc_macro_attribute]
pub fn data(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let data = parse_macro_input!(input as Item);

    let vis = match &data {
        Item::Enum(item) => &item.vis,
        Item::Struct(item) => &item.vis,
        _ => error!(data, "the attribute must only be used on an enum or struct definition"),
    };

    if !matches!(vis, Visibility::Public(_)) {
        error!(vis, "the item must be public");
    }

    let tokens = quote! {
        #[derive(::wasmql::serde::Deserialize, ::wasmql::serde::Serialize)]
        #[serde(crate = "::wasmql::serde")]
        #data
    };

    tokens.into()
}
