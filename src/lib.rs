//! WasmQL

#![feature(doc_cfg)]
#![cfg_attr(target_arch = "wasm32", no_std)]
#![cfg_attr(doc_cfg, feature(doc_cfg))]

#![allow(unused)] // todo: remove this

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
    //! Since it is **highly adivsed** to use wasmql in a `no_std` environment (to reduce
    //! binary file), you will be missing a few common rust types like [`Box`], [`Vec`] or [`String`].
    //! 
    //! This module simply re-exports those items from the `alloc` crate.

    pub use alloc::borrow::ToOwned;
    pub use alloc::boxed::Box;
    pub use alloc::string::String;
    pub use alloc::string::ToString;
    pub use alloc::vec::Vec;
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg_attr(doc_cfg, doc(cfg(non(target_arch = "wasm32"))))]
pub mod error {
    //! Error handling for the backend-side.
    //! 
    //! This module exposes the [`Error`] struct and the
    //! [`Result`] type alias. Both are used to signify errors
    //! during decoding/encoding of queries.

    use thiserror::Error;

    /// An error that may occur during dipatching of a query
    /// to a codec handler.
    /// 
    /// See the individual variants documentations for additional
    /// information about the possible errors.
    #[doc(cfg(not(target_arch = "wasm32")))]
    #[derive(Clone, Eq, PartialEq, Debug, Error)]
    pub enum Error {
        /// Error during decoding/encoding of the raw bytes
        /// of a query to/from a rust type.
        #[error("codec error: {0}")]
        Postcard(postcard::Error),
        /// The last two bytes of a raw query are used to dispatch
        /// the query to the correct method of its handler. This error is returned
        /// when the query is smaller than two bytes long, or when those
        /// last two bytes point to an invalid method.
        #[error("dispatch error")]
        DispatchError,
    }

    #[doc(cfg(not(target_arch = "wasm32")))]
    impl From<postcard::Error> for Error {
        fn from(err: postcard::Error) -> Self {
            Self::Postcard(err)
        }
    }

    /// Convenient alias for a standard library [`Result`](std::result::Result), with
    /// its `E` generic set to [`Error`].
    pub type Result<T> = std::result::Result<T, Error>;
}

#[doc(hidden)]
pub use serde;

pub use wasmql_macros::{api, data};
