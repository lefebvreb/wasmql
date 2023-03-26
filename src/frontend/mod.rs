mod alloc;
mod de;
mod js;
mod panic;
mod ser;

use core::slice;

use serde::de::DeserializeOwned;
use serde::Serialize;

pub use self::js::JsValue;

pub unsafe fn decode<T: Serialize + DeserializeOwned + core::fmt::Debug>() -> JsValue {
    let (data, len) = js::get_buffer();
    let bytes = slice::from_raw_parts(data, len);
    js::log(&::alloc::format!("[WASM] data={data:?} len={len:?} bytes={bytes:?}"));
    let val: T = postcard::from_bytes(bytes).unwrap();
    js::log(&::alloc::format!("[WASM] decoded={val:?}"));
    ser::to_js(&val).unwrap()
}

pub fn encode<T: Serialize + DeserializeOwned + core::fmt::Debug>(val: JsValue, code: u16) -> JsValue {
    let val: T = de::from_js(val).unwrap();
    js::log(&::alloc::format!("[WASM] encoded={val:?}"));
    let mut bytes = postcard::to_allocvec(&val).unwrap();
    bytes.extend(&code.to_le_bytes());
    JsValue::from_bytes(&bytes)
}
