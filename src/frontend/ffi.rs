//! All functions related to interfacing with javascript.

use core::fmt;

use super::js::JsValue;

mod exports {
    use core::alloc::Layout;

    use alloc::alloc::alloc;

    // All exported functions need to be prefixed with a double underscore.
    // User-defined function will be prevented to begin with this prefix.

    /// Allocates some bytes for js to write to.
    #[no_mangle]
    fn __alloc(len: usize) -> *const u8 {
        let layout = Layout::array::<u8>(len).unwrap();
        unsafe { alloc(layout) }
    }

    /// Resets the allocator.
    /// 
    /// # Safety
    /// 
    /// The caller must ensure that no allocated objects currently exist in the program.
    #[no_mangle]
    unsafe fn __reset() {
        super::super::alloc::reset();
    }
}

mod imports {
    use super::*;

    extern "C" {
        pub fn number(v: f64) -> JsValue;
    
        pub fn string(ptr: *const u8, len: usize) -> JsValue;
    
        pub fn bytes(ptr: *const u8, len: usize) -> JsValue;
    
        pub fn log(v: JsValue);
    }
}

pub const fn boolean(v: bool) -> JsValue {
    JsValue(v as u32)
}

pub const fn null() -> JsValue {
    JsValue(2)
}

pub const fn undefined() -> JsValue {
    JsValue(3)
}

pub fn number(v: f64) -> JsValue {
    unsafe { imports::number(v) }
}

pub fn string(v: &str) -> JsValue {
    unsafe { imports::string(v.as_ptr(), v.len()) }
}

pub fn bytes(v: &[u8]) -> JsValue {
    unsafe { imports::string(v.as_ptr(), v.len()) }
}

impl fmt::Display for JsValue {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unsafe { imports::log(*self) };
        Ok(())
    }
}
