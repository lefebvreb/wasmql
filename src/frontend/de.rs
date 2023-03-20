//! Deserializing from js value to rust type.

use serde::de::{Visitor, DeserializeOwned};
use serde::Deserializer;

use super::ffi;
use super::js::{JsValue, JsResult};

impl<'de> Deserializer<'de> for &'de mut JsValue {
    type Error = JsValue;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        unimplemented!()
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_bool(ffi::as_boolean(*self))
    }

    fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_i8(ffi::as_number(*self) as i8)
    }

    fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_i16(ffi::as_number(*self) as i16)
    }

    fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_i32(ffi::as_number(*self) as i32)
    }

    fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_i64(ffi::as_number(*self) as i64)
    }

    fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_u8(ffi::as_number(*self) as u8)
    }

    fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_u16(ffi::as_number(*self) as u16)
    }

    fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_u32(ffi::as_number(*self) as u32)
    }

    fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_u64(ffi::as_number(*self) as u64)
    }

    fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_f32(ffi::as_number(*self) as f32)
    }

    fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_f64(ffi::as_number(*self) as f64)
    }

    fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_char(ffi::as_char(*self))
    }

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_string(ffi::as_string(*self))
    }

    fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        self.deserialize_byte_buf(visitor)
    }

    fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_byte_buf(ffi::as_bytes(*self))
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        if ffi::is_null(*self) {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> JsResult<V::Value> {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> JsResult<V::Value> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        todo!()
    }

    fn deserialize_tuple<V: Visitor<'de>>(self, len: usize, visitor: V) -> JsResult<V::Value> {
        todo!()
    }

    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> JsResult<V::Value> {
        todo!()
    }

    fn deserialize_map<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        todo!()
    }

    fn deserialize_struct<V: Visitor<'de>>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> JsResult<V::Value> {
        todo!()
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> JsResult<V::Value> {
        todo!()
    }

    fn deserialize_identifier<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        todo!()
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        todo!()
    }
}

pub fn from_js<T: DeserializeOwned>(mut val: JsValue) -> JsResult<T> {
    T::deserialize(&mut val)
}
