// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, Item, Visibility};

// #[cfg(feature = "frontend")]
// #[proc_macro_attribute]
// pub fn data(_attr: TokenStream, input: TokenStream) -> TokenStream {
//     let data = parse_macro_input!(input as Item);

//     let vis = match &data {
//         Item::Enum(item) => &item.vis,
//         Item::Struct(item) => &item.vis,
//         Item::Union(item) => &item.vis,
//         _ => panic!("the wasmql::data attribute must only be used on an enum, struct or union definition"),
//     };

//     if !matches!(vis, Visibility::Public(_)) {
//         panic!("the wasmql::data attribute must only be used on a public definition");
//     }

//     let tokens = quote! {
//         #[derive(::wasmql::rkyv::Deserialize, ::wasmql::serde::Serialize)]
//     };

//     tokens.into()
// }

// #[cfg(not(feature = "frontend"))]
// #[proc_macro_attribute]
// pub fn data(_attr: TokenStream, input: TokenStream) -> TokenStream {
//     input
// }
