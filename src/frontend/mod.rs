mod alloc;
mod de;
mod js;
mod panic;
mod ser;

use core::slice;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub use self::js::JsValue;

pub unsafe fn decode<T: Serialize + DeserializeOwned>() -> JsValue {
    let (data, len) = js::get_buffer();
    let bytes = slice::from_raw_parts(data, len);
    let val: T = postcard::from_bytes(bytes).unwrap();
    ser::to_js(&val).unwrap()
}

pub fn encode<T: Serialize + DeserializeOwned>(val: JsValue, code: u16) -> JsValue {
    let val: T = de::from_js(val).unwrap();
    let mut bytes = postcard::to_allocvec(&val).unwrap();
    bytes.extend(&code.to_le_bytes());
    JsValue::from_bytes(&bytes)
}
