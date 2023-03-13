use serde::de::DeserializeOwned;
use serde::Serialize;

use self::de::JsDeserializer;
use self::js::JsValue;
use self::ser::JsSerializer;

mod alloc;
mod de;
mod ffi;
mod js;
mod ser;

pub fn decode<T: Serialize + DeserializeOwned>() -> Result<JsValue, JsValue> {
    let bytes = ffi::buffer();
    let val: T = postcard::from_bytes(bytes)?;
    val.serialize(JsSerializer)
}

pub fn encode<T: Serialize + DeserializeOwned>(v: JsValue) -> Result<JsValue, JsValue> {
    let mut deserializer = JsDeserializer(v);
    let val = T::deserialize(&mut deserializer)?;
    let bytes = postcard::to_allocvec(&val)?;
    Ok()
}
