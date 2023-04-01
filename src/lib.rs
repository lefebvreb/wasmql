#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![no_std]

#![doc = include_str!("../README.md")]
//! # Wasm Backend
//! 
//! By default, a WasmQL codec compiled to a wasm target family (such as wasm32 or 
//! wasm64 and including wasm-wasi), will be treated as for the frontend and such
//! generate frontend code.
//! 
//! On rare occasions, you may wish to compile your backend to wasm, here is how to do it:
//! 1. In your codec's crate `Cargo.toml` file, add a feature named "wasm-backend", just like that:
//! ```toml
//! [features]
//! wasm-backend = []
//! ```
//! 2. In your backend, activate the "wasm-backend" feature for both the `wasmql` crate and
//! your codec's crate.

extern crate alloc;

#[doc(hidden)]
#[cfg(all(target_family = "wasm", not(feature = "wasm-backend")))]
pub mod frontend;

#[doc(hidden)]
#[cfg(any(not(target_family = "wasm"), feature = "wasm-backend"))]
pub mod backend;

#[cfg(any(not(target_family = "wasm"), feature = "wasm-backend"))]
mod error;

#[cfg(any(not(target_family = "wasm"), feature = "wasm-backend"))]
pub use error::*;

pub mod prelude {
    //! `use wasmql::prelude::*;` to import common types missing in `no_std` environments.
    //!
    //! Since it is **highly adivsed** to use WasmQL in a `no_std` environment (to reduce
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
