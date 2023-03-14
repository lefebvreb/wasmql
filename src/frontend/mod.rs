use core::slice::from_raw_parts;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub use self::js::JsValue;
use self::js::JsResult;

mod alloc;
mod de;
mod ffi;
mod js;
mod ser;


pub unsafe fn decode<T: Serialize + DeserializeOwned>(ptr: *const u8, len: usize) -> JsResult<JsValue> {
    let bytes = from_raw_parts(ptr, len);
    let val: T = postcard::from_bytes(bytes)?;
    ser::to_js(&val)
}

pub fn encode<T: Serialize + DeserializeOwned>(val: JsValue) -> Result<JsValue, JsValue> {
    let val = de::from_js(val)?;
    let bytes = postcard::to_allocvec(&val)?;
    Ok(ffi::bytes(&bytes))
}
