#![no_std]

extern crate alloc;

pub use wasmql_macros::data;

#[doc(hidden)]
#[cfg(feature = "frontend")]
pub mod frontend;

#[doc(hidden)]
#[cfg(not(feature = "frontend"))]
pub mod backend;
