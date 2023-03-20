use core::fmt::Display;
use core::panic::PanicInfo;

use alloc::fmt;
use alloc::string::ToString;
use serde::{ser, de};

use super::{ffi, JsValue};

pub enum Throw {}

/// A result that can be passed back to js.
pub type ThrowOr<T> = Result<T, Throw>;

impl fmt::Debug for Throw {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

impl fmt::Display for Throw {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

impl ser::Error for Throw {
    fn custom<T: Display>(msg: T) -> Self {
        panic!("serialization error")
    }
}

impl de::Error for Throw {
    fn custom<T: Display>(msg: T) -> Self {
        panic!("deserialization error")
    }
}

impl From<postcard::Error> for Throw {
    fn from(err: postcard::Error) -> Self {
        panic!("postcard error")
    }
}


#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    match info.payload().downcast_ref::<&str>() {
        Some(s) => JsValue::from_string(s),
        _ => JsValue::undefined(),
    }.throw()
}