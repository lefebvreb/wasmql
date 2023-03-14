//! All functions related to interfacing with javascript.

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
        /// Creates a new `number`.
        pub fn number(v: f64) -> JsValue;
        /// Creates a new `string`.
        pub fn string(ptr: *const u8, len: usize) -> JsValue;
        /// Creates a new `Uint8Array`.
        pub fn bytes(ptr: *const u8, len: usize) -> JsValue;
        /// Creates a new empty object `{}`.
        pub fn object() -> JsValue;
        /// Appends a new key-value pair to an `object`.
        pub fn object_append(obj: JsValue, k: JsValue, v: JsValue) -> JsValue;
        /// Creates a new array `[]`.
        pub fn array() -> JsValue;
        /// Appends a new value to an `array`.
        pub fn array_append(arr: JsValue, v: JsValue) -> JsValue;
        /// Logs a js-owned value using `console.log`.
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
    unsafe { imports::bytes(v.as_ptr(), v.len()) }
}

pub fn object() -> JsValue {
    unsafe { imports::object() }
}

pub fn object_append(obj: JsValue, k: JsValue, v: JsValue) -> JsValue {
    unsafe { imports::object_append(obj, k, v) }
}

pub fn array() -> JsValue {
    unsafe { imports::array() }
}

pub fn array_append(arr: JsValue, v: JsValue) -> JsValue {
    unsafe { imports::array_append(arr, v) }
}

pub fn log(v: JsValue) {
    unsafe { imports::log(v) }
}
