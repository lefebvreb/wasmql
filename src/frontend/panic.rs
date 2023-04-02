use core::fmt::Display;
use core::panic::PanicInfo;

use alloc::fmt;
use serde::{de, ser};

use super::JsValue;

pub enum Throw {}

/// A result that can be passed back to js.
pub type Result<T> = core::result::Result<T, Throw>;

impl fmt::Debug for Throw {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unreachable!()
    }
}

impl fmt::Display for Throw {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unreachable!()
    }
}

impl ser::Error for Throw {
    fn custom<T: Display>(msg: T) -> Self {
        #[cfg(feature = "debug-panic-fmt")]
        panic!("serialization error: {msg}");
        #[cfg(not(feature = "debug-panic-fmt"))]
        panic!();
    }
}

impl de::Error for Throw {
    fn custom<T: Display>(msg: T) -> Self {
        #[cfg(feature = "debug-panic-fmt")]
        panic!("deserialization error: {msg}");
        #[cfg(not(feature = "debug-panic-fmt"))]
        panic!();
    }
}

impl From<postcard::Error> for Throw {
    fn from(err: postcard::Error) -> Self {
        #[cfg(feature = "debug-panic-fmt")]
        panic!("postcared error: {err}");
        #[cfg(not(feature = "debug-panic-fmt"))]
        panic!();
    }
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    #[cfg(feature = "debug-panic-fmt")]
    JsValue::from_string(&alloc::format!("{info}")).throw();
    #[cfg(not(feature = "debug-panic-fmt"))]
    JsValue::from_string("WasmQL type error").throw();
}
