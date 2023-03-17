#![cfg_attr(target_arch = "wasm32", no_std)]

#![allow(unused)]

#[cfg(target_arch = "wasm32")]
extern crate alloc;

#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub mod frontend;

#[doc(hidden)]
#[cfg(not(target_arch = "wasm32"))]
pub mod backend;

#[cfg(any(doc, not(target_arch = "wasm32")))]
pub mod build;

#[doc(hidden)]
pub use serde;

pub use wasmql_macros::{api, data};
