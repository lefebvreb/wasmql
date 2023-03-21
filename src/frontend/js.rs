//! All functions related to interfacing with javascript.

use alloc::string::String;
use alloc::vec::Vec;

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

    // All of these functions are free to throw an exception
    // (which will pass through wasm but not be intercepted).
    extern "C" {
        /* boolean */

        /// Gets the value as a `bool`.
        pub fn as_boolean(val: JsValue) -> bool;

        /* null */

        /// Tests if the value is a `null`.
        pub fn is_null(val: JsValue) -> bool;

        /* number */

        /// Creates a new `number`.
        pub fn from_number(num: f64) -> JsValue;
        /// Gets the value as a `f64`.
        pub fn as_number(val: JsValue) -> f64;

        /* string */

        /// Creates a new `string`.
        pub fn from_string(ptr: *const u8, len: usize) -> JsValue;
        /// Gets the length of a js-owned `string`.
        pub fn string_len(val: JsValue) -> usize;
        /// Gets the utf-8 encoded bytes of a js-owned `string`.
        pub fn as_string(val: JsValue) -> *mut u8;
        /// Gets the value as an utf-8 code point.
        pub fn as_char(val: JsValue) -> u32;

        /* bytes */
        
        /// Creates a new `ArrayBuffer`.
        pub fn from_bytes(ptr: *const u8, len: usize) -> JsValue;
        /// Gets the length of a js-owned `ArrayBuffer`.
        pub fn bytes_len(bytes: JsValue) -> usize;
        /// Gets the bytes of an `ArrayBuffer`.
        pub fn as_bytes(bytes: JsValue) -> *mut u8;

        /* object */

        /// Creates a new empty object `{}`.
        pub fn new_object() -> JsValue;
        /// Appends a new key-value pair to an `object`.
        pub fn object_append(obj: JsValue, key: JsValue, val: JsValue);

        /* array */

        /// Creates a new array `[]`.
        pub fn new_array() -> JsValue;
        /// Appends a new value to an `array`.
        pub fn array_append(arr: JsValue, val: JsValue);
        /// Returns the len of an `array`.
        pub fn array_len(arr: JsValue) -> usize;
        /// Gets the ith element of an `array`.
        pub fn array_get(arr: JsValue, i: usize) -> JsValue;

        /* iter */

        /// Returns a js `array` resulting from `Object.entries(obj)`, called here an iter.
        /// One can use `array_len` to get the length of this iter.
        pub fn new_iter(obj: JsValue) -> JsValue;
        /// Returns the ith key of this iterator.
        pub fn iter_key(iter: JsValue, i: usize) -> JsValue;
        /// Returns the ith value of this iterator.
        pub fn iter_val(iter: JsValue, i: usize) -> JsValue;
          
        /* throw */

        /// Throws the given js-owned value using a `throw` statement, and resets wasm memory.
        pub fn throw(val: JsValue);
    }
}

/// The index of a js-owned value, stored in a js-side array.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct JsValue(pub(crate) u32);

impl JsValue {
    /* boolean */

    pub const fn from_bool(v: bool) -> Self {
        Self(v as u32)
    }
    
    pub fn as_boolean(self) -> bool {
        unsafe { imports::as_boolean(self) }
    }
    
    /* null */

    pub const fn null() -> Self {
        Self(2)
    }

    pub fn is_null(self) -> bool {
        unsafe { imports::is_null(self) }
    }

    /* undefined */
    
    pub const fn undefined() -> Self {
        Self(3)
    }

    /* number */

    pub fn from_number(num: f64) -> Self {
        unsafe { imports::from_number(num) }
    }
    
    pub fn as_number(self) -> f64 {
        unsafe { imports::as_number(self) }
    }

    /* string */

    pub fn from_string(str: &str) -> Self {
        unsafe { imports::from_string(str.as_ptr(), str.len()) }
    }
    
    pub fn as_string(self) -> String {
        unsafe { 
            let len = imports::string_len(self);
            let ptr = imports::as_string(self);
            String::from_raw_parts(ptr, len, len)
        }
    }
    
    pub fn as_char(self) -> char {
        unsafe { 
            let code = imports::as_char(self);
            char::from_u32_unchecked(code)
        }
    }

    /* bytes */  

    pub fn from_bytes(bytes: &[u8]) -> Self {
        unsafe { imports::from_bytes(bytes.as_ptr(), bytes.len()) }
    }
    
    pub fn as_bytes(self) -> Vec<u8> {
        unsafe {
            let len = imports::bytes_len(self);
            let ptr = imports::as_bytes(self);
            Vec::from_raw_parts(ptr, len, len)
        }
    }

    /* object */  

    pub fn new_object() -> Self {
        unsafe { imports::new_object() }
    }
    
    pub fn object_append(self, key: JsValue, val: JsValue) {
        unsafe { imports::object_append(self, key, val) }
    }

    /* array */
    
    pub fn new_array() -> Self {
        unsafe { imports::new_array() }
    }
    
    pub fn array_append(self, val: JsValue) {
        unsafe { imports::array_append(self, val) }
    }
    
    pub fn array_len(self) -> usize {
        unsafe { imports::array_len(self) }
    }
    
    pub fn array_get(self, i: usize) -> JsValue {
        unsafe { imports::array_get(self, i) }
    }

    /* iter */

    pub fn new_iter(obj: JsValue) -> JsValue {
        unsafe { imports::new_iter(obj) }
    }

    pub fn iter_key(self, i: usize) -> JsValue {
        unsafe { imports::iter_key(self, i) }
    }

    pub fn iter_val(self, i: usize) -> JsValue {
        unsafe { imports::iter_val(self, i) }
    }

    /* throw */

    pub fn throw(self) -> ! {
        unsafe { imports::throw(self) }
        unreachable!()
    }
}
