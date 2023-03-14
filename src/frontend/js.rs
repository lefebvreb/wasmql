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
        ffi::string(&v.to_string())
    }
}

impl fmt::Debug for JsValue {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        ffi::log(*self);
        Ok(())
    }
}

impl fmt::Display for JsValue {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        ffi::log(*self);
        Ok(())
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
