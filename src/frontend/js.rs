//! Js values are simply ids of values stored in a js array and owned by the host
//! js host environment.
//! 
//! Special values are:
//! - 

use core::fmt::Display;

use alloc::string::ToString;
use serde::{ser, de};

use super::ffi;

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct JsValue(pub(crate) u32);

pub type JsResult<T> = Result<T, JsValue>;

impl JsValue {
    fn from_display<T: Display>(v: T) -> Self {
        ffi::string(&v.to_string())
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
