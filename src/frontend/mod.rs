mod alloc;
mod de;
mod ffi;
mod js;
mod ser;

use core::slice;

use serde::de::DeserializeOwned;
use serde::Serialize;

use self::js::UnwrapOrThrow;

pub use self::js::JsValue;

pub unsafe fn decode<T: Serialize + DeserializeOwned>(ptr: *mut u8, len: usize) -> JsValue {
    let bytes = slice::from_raw_parts(ptr, len);
    let val: T = postcard::from_bytes(bytes).unwrap_or_throw();
    ser::to_js(&val).unwrap_or_throw()
}

pub fn encode<T: Serialize + DeserializeOwned>(val: JsValue) -> JsValue {
    let val: T = de::from_js(val).unwrap_or_throw();
    let bytes = postcard::to_allocvec(&val).unwrap_or_throw();
    ffi::from_bytes(&bytes)
}
