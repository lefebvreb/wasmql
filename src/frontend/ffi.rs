//! All functions related to interfacing with javascript.

use super::js::JsValue;

mod exports {
    use core::alloc::Layout;

    use alloc::alloc::alloc;

    // All exported functions need to be prefixed with a double underscore.
    // User-defined function will be prevented to begin with this prefix.

    /// Allocates some bytes for js to write to.
    #[no_mangle]
    unsafe fn __alloc(len: usize) -> *const u8 {
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
        // Creators/Mutators
        /// Creates a new `number`.
        pub fn number(num: f64) -> JsValue;
        /// Creates a new `string`.
        pub fn string(ptr: *const u8, len: usize) -> JsValue;
        /// Creates a new `Uint8Array`.
        pub fn bytes(ptr: *const u8, len: usize) -> JsValue;
        /// Creates a new empty object `{}`.
        pub fn object() -> JsValue;
        /// Appends a new key-value pair to an `object`.
        pub fn object_append(this: JsValue, key: JsValue, val: JsValue) -> JsValue;
        /// Creates a new array `[]`.
        pub fn array() -> JsValue;
        /// Appends a new value to an `array`.
        pub fn array_append(this: JsValue, val: JsValue) -> JsValue;

        // Procedures
        /// Logs a js-owned value using `console.log`.
        pub fn log(val: JsValue);
        /// Throws the given js-owned value using a `throw` statement, and reset wasm memory.
        pub fn throw(val: JsValue);
    }
}

pub const fn from_boolean(v: bool) -> JsValue {
    JsValue(v as u32)
}

pub const fn null() -> JsValue {
    JsValue(2)
}

pub const fn undefined() -> JsValue {
    JsValue(3)
}

pub fn from_number(num: f64) -> JsValue {
    unsafe { imports::number(num) }
}

pub fn from_string(str: &str) -> JsValue {
    unsafe { imports::string(str.as_ptr(), str.len()) }
}

pub fn from_bytes(bytes: &[u8]) -> JsValue {
    unsafe { imports::bytes(bytes.as_ptr(), bytes.len()) }
}

pub fn new_object() -> JsValue {
    unsafe { imports::object() }
}

pub fn object_append(this: JsValue, key: JsValue, val: JsValue) -> JsValue {
    unsafe { imports::object_append(this, key, val) }
}

pub fn array() -> JsValue {
    unsafe { imports::array() }
}

pub fn array_append(this: JsValue, val: JsValue) -> JsValue {
    unsafe { imports::array_append(this, val) }
}

// pub fn to_bytes(val: JsValue) -> &'static [u8] {
//     unsafe {
//         let ptr = imports::bytes_ptr(val);
//         let len = imports::bytes_len(val);
//         slice::from_raw_parts(ptr, len)
//     }
// }

pub fn log(val: JsValue) {
    unsafe { imports::log(val) }
}

pub fn throw(val: JsValue) -> ! {
    unsafe { imports::throw(val) }
    unreachable!()
}