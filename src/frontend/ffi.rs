//! All functions related to interfacing with javascript.

use core::fmt;

use alloc::vec::Vec;

use super::js::JsValue;

// All foreign functons need to be prefixed with a double underscore.
// User-defined function will be prevented to begin with this prefix.
extern "C" {
    // Constructors

    fn __bytes(ptr: u32, len: u32) -> JsValue;

    fn __string(ptr: u32, len: u32) -> JsValue;

    // Accessors

    // Other

    fn __log(v: JsValue);
}

static mut BUFFER: Vec<u8> = Vec::new();

#[no_mangle]
pub fn _alloc(len: u32) -> u32 {
    unsafe {
        BUFFER.clear();
        BUFFER.reserve(len as usize);
        BUFFER.set_len(len as usize);
        BUFFER.as_ptr() as u32
    }
}

pub fn buffer() -> &'static [u8] {
    unsafe { &BUFFER }
}

fn raw_str(v: &[u8]) -> (u32, u32) {
    (v.as_ptr() as u32, v.len() as u32)
}

impl JsValue {
    pub fn from_bytes(v: &[u8]) -> JsValue {
        let (ptr, len) = raw_str(v);
        unsafe { __bytes(ptr, len) }
    }

    pub fn from_string(v: &str) -> Self {
        let (ptr, len) = raw_str(v.as_bytes());
        unsafe { __string(ptr, len) }
    }
}

impl fmt::Display for JsValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe { __log(*self) };
        Ok(())
    }
}
