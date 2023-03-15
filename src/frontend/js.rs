//! Js values are simply ids of values stored in a js array and owned by the host
//! js host environment.

use core::fmt::{self, Display};

use alloc::string::ToString;
use serde::{ser, de};

use super::ffi;

/// The index of a js-owned value, stored in a js-side array.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct JsValue(pub(crate) u32);

/// A result that can be passed back to js.
pub type JsResult<T> = Result<T, JsValue>;

impl JsValue {
    pub(crate) fn from_display<T: Display>(v: T) -> Self {
        ffi::from_string(&v.to_string())
    }
}

impl fmt::Debug for JsValue {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

impl fmt::Display for JsValue {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

impl ser::Error for JsValue {
    fn custom<T: Display>(msg: T) -> Self {
        Self::from_display(msg)
    }
}

impl de::Error for JsValue {
    fn custom<T: Display>(msg: T) -> Self {
        Self::from_display(msg)
    }
}

impl From<postcard::Error> for JsValue {
    fn from(err: postcard::Error) -> Self {
        Self::from_display(err)
    }
}

pub trait UnwrapOrThrow<T> {
    fn unwrap_or_throw(self) -> T;
} 

impl<T, E: Into<JsValue>> UnwrapOrThrow<T> for Result<T, E> {
    fn unwrap_or_throw(self) -> T {
        match self {
            Ok(val) => val,
            Err(err) => ffi::throw(err.into()),
        }
    }
}
