//! Js values are simply ids of values stored in a js array and owned by the host
//! js host environment.

use core::fmt::Display;

use alloc::string::ToString;
use serde::{ser, de};

#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct JsValue(u32);

pub type JsResult<T> = Result<T, JsValue>;

impl JsValue {
    pub fn from_display<T: Display>(v: T) -> Self {
        Self::from_string(&v.to_string())
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
