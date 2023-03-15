#![no_std]

extern crate alloc;

#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub mod frontend;

#[doc(hidden)]
#[cfg(not(target_arch = "wasm32"))]
pub mod backend;

#[doc(hidden)]
pub use serde;

pub use wasmql_macros::{api, data};
