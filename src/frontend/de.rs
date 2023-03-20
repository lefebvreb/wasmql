//! Deserializing from js value to rust type.

use serde::de::{Visitor, DeserializeOwned, SeqAccess, DeserializeSeed};
use serde::Deserializer;

use super::ffi;
use super::js::{JsValue, JsResult};

struct JsSeqDeserializer {
    arr: JsValue,
    index: usize,
}

impl<'de> Deserializer<'de> for &'de mut JsValue {
    type Error = JsValue;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        unimplemented!()
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_bool(self.as_boolean())
    }

    fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_i8(self.as_number() as i8)
    }

    fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_i16(self.as_number() as i16)
    }

    fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_i32(self.as_number() as i32)
    }

    fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_i64(self.as_number() as i64)
    }

    fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_u8(self.as_number() as u8)
    }

    fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_u16(self.as_number() as u16)
    }

    fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_u32(self.as_number() as u32)
    }

    fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_u64(self.as_number() as u64)
    }

    fn deserialize_f32<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_f32(self.as_number() as f32)
    }

    fn deserialize_f64<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_f64(self.as_number() as f64)
    }

    fn deserialize_char<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_char(self.as_char())
    }

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_string(self.as_string())
    }

    fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        self.deserialize_byte_buf(visitor)
    }

    fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        visitor.visit_byte_buf(self.as_bytes())
    }

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> JsResult<V::Value> {
        if self.is_null() {
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
        // visitor.visit_seq(JsSeqDeserializer {
        //     arr: *self,
        //     index: 0,
        // })
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

impl<'a> SeqAccess<'a> for JsSeqDeserializer {
    type Error = JsValue;

    fn next_element_seed<T: DeserializeSeed<'a>>(&mut self, _seed: T) -> JsResult<Option<T::Value>> {
        todo!()
    }
}

pub fn from_js<T: DeserializeOwned>(mut val: JsValue) -> JsResult<T> {
    T::deserialize(&mut val)
}
