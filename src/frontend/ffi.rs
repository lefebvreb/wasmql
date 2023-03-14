//! All functions related to interfacing with javascript.

use core::alloc::Layout;
use core::fmt;

use alloc::alloc::alloc;

use super::js::JsValue;

mod env {
    use super::*;

    extern "C" {
        pub fn bytes(ptr: *const u8, len: usize) -> JsValue;
    
        pub fn string(ptr: *const u8, len: usize) -> JsValue;
    
        pub fn log(v: JsValue);
    }
}

// All exported functions need to be prefixed with a double underscore.
// User-defined function will be prevented to begin with this prefix.

#[no_mangle]
fn __alloc(len: usize) -> *const u8 {
    let layout = Layout::array::<u8>(len).unwrap();
    unsafe { alloc(layout) }
}

#[no_mangle]
fn __reset(len: usize) -> *const u8 {
    let layout = Layout::array::<u8>(len).unwrap();
    unsafe { alloc(layout) }
}

impl JsValue {
    pub fn from_bytes(v: &[u8]) -> JsValue {
        unsafe { env::bytes(v.as_ptr(), v.len()) }
    }

    pub fn from_string(v: &str) -> Self {
        unsafe { env::string(v.as_ptr(), v.len()) }
    }
}

impl fmt::Display for JsValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe { env::log(*self) };
        Ok(())
    }
}
