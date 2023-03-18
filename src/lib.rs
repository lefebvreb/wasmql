//! WasmQL

#![allow(unused)] // todo: remove this
#![no_std]

extern crate alloc;

#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub mod frontend;

#[doc(hidden)]
#[cfg(not(target_arch = "wasm32"))]
pub mod backend;

pub mod prelude {
    //! `use wasmql::prelude::*;` to import types missing in `no_std` environment.
    //! 
    //! Since it is *highly adivsed* to use wasmql in a `no_std` environment (to reduce
    //! binary file), you will be missing a few common rust types like [`Box`], [`Vec`] or [`String`].
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

pub use wasmql_macros::{api, data};
