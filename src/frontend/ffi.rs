//! All functions related to interfacing with javascript.

use alloc::string::String;
use alloc::vec::Vec;

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

    /// All of this functions are free to throw an error
    /// (which will pass through wasm but not be intercepted).
    extern "C" {
        /// Creates a new `number`.
        pub fn from_number(num: f64) -> JsValue;
        /// Creates a new `string`.
        pub fn from_string(ptr: *const u8, len: usize) -> JsValue;
        /// Creates a new `ArrayBuffer`.
        pub fn from_bytes(ptr: *const u8, len: usize) -> JsValue;
        /// Creates a new empty object `{}`.
        pub fn from_object() -> JsValue;
        /// Creates a new array `[]`.
        pub fn from_array() -> JsValue;

        /// Gets the value as a `bool`.
        pub fn to_boolean(val: JsValue) -> bool;
        /// Gets the value as a `f64`.
        pub fn to_number(val: JsValue) -> f64;
        /// Gets the value as an utf-8 code point.
        pub fn to_char(val: JsValue) -> u32;
        /// Gets the utf-8 encoded bytes of a js-owned `string`.
        pub fn to_string(val: JsValue) -> *mut u8;
        /// Gets the bytes of an `ArrayBuffer`.
        pub fn to_bytes(val: JsValue) -> *mut u8;
        
        /// Tests if the value is a `null`.
        pub fn is_null(val: JsValue) -> bool;
        /// Appends a new key-value pair to an `object`.
        pub fn object_append(obj: JsValue, key: JsValue, val: JsValue);
        /// Appends a new value to an `array`.
        pub fn array_append(obj: JsValue, val: JsValue);
        /// Gets the length of a js-owned `string`.
        pub fn string_len(val: JsValue) -> usize;
        /// Gets the length of a js-owned `ArrayBuffer`.
        pub fn bytes_len(val: JsValue) -> usize;

        /// Throws the given js-owned value using a `throw` statement, and resets wasm memory.
        pub fn throw(val: JsValue);
    }
}

impl JsValue {
    pub const fn from_boolean(v: bool) -> Self {
        Self(v as u32)
    }
    
    pub const fn null() -> Self {
        Self(2)
    }
    
    pub const fn undefined() -> Self {
        Self(3)
    }

    pub fn from_number(num: f64) -> Self {
        unsafe { imports::from_number(num) }
    }

    pub fn from_string(str: &str) -> Self {
        unsafe { imports::from_string(str.as_ptr(), str.len()) }
    }
    
    pub fn from_bytes(bytes: &[u8]) -> Self {
        unsafe { imports::from_bytes(bytes.as_ptr(), bytes.len()) }
    }
    
    pub fn new_object() -> Self {
        unsafe { imports::from_object() }
    }
    
    pub fn new_array() -> Self {
        unsafe { imports::from_array() }
    }
    
    pub fn as_boolean(self) -> bool {
        unsafe { imports::to_boolean(self) }
    }
    
    pub fn is_null(self) -> bool {
        unsafe { imports::is_null(self) }
    }
    
    pub fn as_number(self) -> f64 {
        unsafe { imports::to_number(self) }
    }
    
    pub fn as_char(self) -> char {
        unsafe { 
            let code = imports::to_char(self);
            char::from_u32_unchecked(code)
        }
    }
    
    pub fn as_string(self) -> String {
        unsafe { 
            let len = imports::string_len(self);
            let ptr = imports::to_string(self);
            String::from_raw_parts(ptr, len, len)
        }
    }
    
    pub fn as_bytes(self) -> Vec<u8> {
        unsafe { 
            let len = imports::bytes_len(self);
            let ptr = imports::to_bytes(self);
            Vec::from_raw_parts(ptr, len, len)
        }
    }
    
    pub fn object_append(obj: JsValue, key: JsValue, val: JsValue) {
        unsafe { imports::object_append(obj, key, val) }
    }
    
    pub fn array_append(obj: JsValue, val: JsValue) {
        unsafe { imports::array_append(obj, val) }
    }

    pub fn throw(self) -> ! {
        unsafe { imports::throw(self) }
        unreachable!()
    }
}
