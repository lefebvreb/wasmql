#![no_std]

extern crate alloc;

#[doc(hidden)]
#[cfg(feature = "frontend")]
pub mod frontend;

#[doc(hidden)]
#[cfg(not(feature = "frontend"))]
pub mod backend;

#[doc(hidden)]
pub use serde::{Serialize, Deserialize};

pub use wasmql_macros::{api, data};
