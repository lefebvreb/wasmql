#![no_std]

//! WasmQL

extern crate alloc;

#[doc(hidden)]
#[cfg(all(target_arch = "wasm32", not(feature = "backend")))]
pub mod frontend;

#[doc(hidden)]
#[cfg(any(not(target_arch = "wasm32"), feature = "backend"))]
pub mod backend;

#[cfg(any(not(target_arch = "wasm32"), feature = "backend"))]
pub mod error;

pub mod prelude {
    //! `use wasmql::prelude::*;` to import common types missing in `no_std` environments.
    //!
    //! Since it is **highly adivsed** to use wasmql in a `no_std` environment (to reduce
    //! binary file size), you will be missing a few common rust types like [`Box`], [`Vec`] or [`String`].
    //!
    //! This module simply re-exports those items from the `alloc` crate.

    pub use alloc::borrow::ToOwned;
    pub use alloc::boxed::Box;
    pub use alloc::string::String;
    pub use alloc::string::ToString;
    pub use alloc::vec::Vec;
}

#[doc(hidden)]
pub use serde;

pub use wasmql_macros::{codec, data};
