#![no_std]

//! WasmQL

extern crate alloc;

#[doc(hidden)]
#[cfg(feature = "frontend")]
pub mod frontend;

#[doc(hidden)]
#[cfg(feature = "backend")]
pub mod backend;

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

#[cfg(feature = "backend")]
pub mod error {
    //! Error handling for the backend-side.
    //! 
    //! This module exposes the [`enum@Error`] struct and the
    //! [`Result`] type alias. Both are used to signify errors
    //! during decoding/encoding of queries.

    use core::fmt;

    /// An error that may occur during dipatching of a query
    /// to a codec handler.
    /// 
    /// See the individual variants documentations for additional
    /// information about the possible errors.
    #[derive(Clone, Eq, PartialEq, Debug)]
    pub enum Error {
        /// Error during decoding/encoding of the raw bytes
        /// of a query to/from a rust type.
        Postcard(postcard::Error),
        /// The last two bytes of a raw query are used to dispatch
        /// the query to the correct method of its handler. This error is returned
        /// when the query is smaller than two bytes long, or when those
        /// last two bytes point to an invalid method.
        Discriminant,
    }

    impl From<postcard::Error> for Error {
        fn from(err: postcard::Error) -> Self {
            Self::Postcard(err)
        }
    }

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Error::Postcard(err) => write!(f, "postcard error: {err}"),
                Error::Discriminant => write!(f, "discriminant error"),
            }
        }
    }

    /// Convenient alias for a standard library [`Result`](std::result::Result), with
    /// its `E` generic set to be [`enum@Error`].
    pub type Result<T> = core::result::Result<T, Error>;
}

#[doc(hidden)]
pub use serde;

pub use wasmql_macros::{api, data};
