//! Error handling for the backend-side.
//!
//! This module exposes the [`enum@Error`] struct and the
//! [`Result`] type alias. Both are used to signify errors
//! during decoding/encoding of queries.

use core::fmt;

/// An error that may occur during dispatch of a query
/// to a codec handler.
///
/// See the individual variants documentations for additional
/// information about the possible errors.
#[cfg_attr(
    doc_cfg,
    doc(cfg(any(not(target_family = "wasm"), feature = "wasm-backend")))
)]
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

/// Convenient alias for a standard library [`Result`](core::result::Result), with
/// its `E` generic type set to be [`enum@Error`].
#[cfg_attr(
    doc_cfg,
    doc(cfg(any(not(target_family = "wasm"), feature = "wasm-backend")))
)]
pub type Result<T> = core::result::Result<T, Error>;
