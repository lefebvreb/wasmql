use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::token::{Comma, Paren};
use syn::{
    parse_macro_input, FnArg, Index, Item, ItemTrait, PatType, Receiver, ReturnType, TraitItem,
    Type, TypeTuple, Visibility,
};

macro_rules! error {
    ($tokens: expr, $message: expr) => {
        return syn::Error::new_spanned($tokens, $message)
            .to_compile_error()
            .into()
    };
}

fn empty_tuple() -> TypeTuple {
    TypeTuple {
        paren_token: Paren::default(),
        elems: Punctuated::new(),
    }
}

/// Attribute for marking a rust trait as defining a WasmQL codec.
/// 
/// It is **highly advised** to make a crate using this macro `#![no_std]`, as
/// that will greatly reduce the size of your binary when compiled to WebAssembly.
///
/// This attribute can be applied to a rust trait that only contains
/// methods, whose signatures are `fn(self, T) -> U` where `T` and
/// `U` are enums or structs marked with the `#[data]` attribute.
///
/// In the backend, the trait will mostly remain as-is, with a single new method
/// added to it, having the signature:
///
/// ```no_run
/// dispatch(self, &[u8]) -> wasmql::Result<Vec<u8>>
/// ```
///
/// This method can be used to decode a binary payload, handle it and encode the result
/// into a `Vec<u8>`.
///
/// In the frontend, this trait will be transformed into a module that exports some functions
/// to JavaScript.
///
/// # Examples
/// 
/// In your codec:
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
/// #[wasmql::codec]
/// pub trait MyApi {
///     fn login(self, login: Login) -> Session;
///
///     fn get_resource(self, session: Session) -> String;
/// }
/// ```
/// 
/// In your backend:
/// 
/// ```no_run
/// struct MyApiImpl;
/// 
/// impl MyApi for MyApiImpl {
///     /* implementation omitted */
/// }
/// 
/// let msg: &[u8] = receive();
/// let res: Vec<u8> = MyApiImpl.dispath(msg)?;
/// send(res);
/// ```
#[proc_macro_attribute]
pub fn codec(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut input: ItemTrait = parse_macro_input!(input);

    let trait_ident = &input.ident;

    if !matches!(input.vis, Visibility::Public(_)) {
        error!(input.vis, "the trait must be public");
    }

    // Wasm funcs exported to js for encoding/decoding.
    let mut extern_funcs = vec![];

    // Arms of the match expression in the backend dispatcher.
    let mut dispatcher_arms = vec![];

    if input.items.len() > u16::MAX as usize {
        error!(input, "the trait may not have more than u16::MAX items");
    }

    // For each item in this trait.
    for (i, item) in input.items.iter().enumerate() {
        let i = i as u16;

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
        let fn_ident = &sig.ident;

        // Name does not begin with "__".
        if fn_ident.to_string().starts_with("__") {
            error!(
                fn_ident,
                "the method identifier must not begin with a double underscore \"__\""
            );
        }

        if *fn_ident == "dispatch" {
            error!(
                fn_ident,
                "the method identifier \"dispatch\" is reserved and cannot be used"
            );
        }

        let mut inputs = sig.inputs.iter();

        // First argument must be `self`.
        let Some(FnArg::Receiver(Receiver { attrs, reference: None, mutability: None, .. })) = inputs.next() else {
            error!(&sig.inputs, "the method's first argument must be self");
        };

        // First argument must not have attributes.
        if let Some(attr) = attrs.first() {
            error!(attr, "the method's inputs must not have any attributes");
        }

        // Argument tuple (excluding self).
        let mut args = empty_tuple();

        // For each subsequent arguments to this trait method.
        for arg in inputs {
            let FnArg::Typed(PatType { attrs, ty, .. }) = arg else { unreachable!() };

            // Argument has no attributes.
            if let Some(attr) = attrs.first() {
                error!(attr, "the method's arguments must not have any attributes");
            }

            args.elems.push(*ty.clone());
        }

        // Make tuple trailing and get indexes over its components.
        let indexes = (0..args.elems.len()).map(Index::from);
        if !args.elems.empty_or_trailing() {
            args.elems.push_punct(Comma::default());
        }

        let ret = match &sig.output {
            ReturnType::Default => Type::Tuple(empty_tuple()),
            ReturnType::Type(_, ty) => *ty.clone(),
        };

        // Arm of the match expression in the dispatcher function.
        dispatcher_arms.push(quote! {
            #i => {
                let arg = ::wasmql::backend::decode::<#args>(bytes)?;
                let ret = self.#fn_ident(#(arg.#indexes),*);
                Ok(::wasmql::backend::encode(&ret)?)
            }
        });

        // Names of the decoder and encoder funcs.
        let decoder = format_ident!("dec_{}", fn_ident);
        let encoder = format_ident!("enc_{}", fn_ident);

        extern_funcs.push(quote! {
            #[no_mangle]
            unsafe fn #decoder() -> ::wasmql::frontend::JsValue {
                ::wasmql::frontend::decode::<#ret>()
            }

            #[no_mangle]
            fn #encoder(val: ::wasmql::frontend::JsValue) -> ::wasmql::frontend::JsValue {
                ::wasmql::frontend::encode::<#args>(val, #i)
            }
        });
    }

    // Append the dispatch function.
    input.items.push({
        let tokens = quote! {
            /// Dispatches a message to this codec.
            ///
            /// This method is provided and does not need to be reimplemented.
            ///
            /// First, this method decodes the raw `bytes` of the message. On success,
            /// the correct handling method is called on `self` with the decoded value. Then,
            /// the result is encoded and returned as a `Vec<u8>`.
            fn dispatch(self, bytes: &[u8]) -> ::wasmql::Result<::wasmql::prelude::Vec<u8>>
            where
                Self: Sized,
            {
                match ::wasmql::backend::discriminant(bytes)? {
                    #(#dispatcher_arms)*
                    _ => Err(::wasmql::Error::Discriminant),
                }
            }
        }
        .into();

        parse_macro_input!(tokens)
    });

    let tokens = quote! {
        #[cfg(all(target_family = "wasm", not(feature = "wasm-backend")))]
        #[allow(non_snake_case)]
        mod #trait_ident {
            use super::*;

            #(#extern_funcs)*
        }

        #[cfg(any(not(target_family = "wasm"), feature = "wasm-backend"))]
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
        _ => error!(
            data,
            "the attribute must only be used on an enum or struct definition"
        ),
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
