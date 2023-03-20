//! Serializing from rust type to js value.

use core::fmt::Display;
use core::mem::size_of;

use serde::{Serializer, Serialize};
use serde::ser::{SerializeSeq, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant, SerializeMap, SerializeStruct, SerializeStructVariant};

use super::ffi;
use super::js::{JsValue, JsResult};

struct JsSerializer;

struct JsTupleVariantSerializer {
    parent: JsValue,
    child: JsValue,
}

struct JsMapSerializer {
    obj: JsValue,
    next_key: Option<JsValue>,
}

struct JsStructVariantSerializer {
    parent: JsValue,
    child: JsValue,
}

impl Serializer for JsSerializer {
    type Ok = JsValue;

    type Error = JsValue;

    type SerializeSeq = JsValue;

    type SerializeTuple = JsValue;

    type SerializeTupleStruct = JsValue;

    type SerializeTupleVariant = JsTupleVariantSerializer;

    type SerializeMap = JsMapSerializer;

    type SerializeStruct = JsValue;

    type SerializeStructVariant = JsStructVariantSerializer;

    fn serialize_bool(self, v: bool) -> JsResult<JsValue> {
        Ok(JsValue::from_bool(v))
    }

    fn serialize_i8(self, v: i8) -> JsResult<JsValue> {
        Ok(JsValue::from_number(v as f64))
    }

    fn serialize_i16(self, v: i16) -> JsResult<JsValue> {
        Ok(JsValue::from_number(v as f64))
    }

    fn serialize_i32(self, v: i32) -> JsResult<JsValue> {
        Ok(JsValue::from_number(v as f64))
    }

    fn serialize_i64(self, v: i64) -> JsResult<JsValue> {
        Ok(JsValue::from_number(v as f64))
    }

    fn serialize_u8(self, v: u8) -> JsResult<JsValue> {
        Ok(JsValue::from_number(v as f64))
    }

    fn serialize_u16(self, v: u16) -> JsResult<JsValue> {
        Ok(JsValue::from_number(v as f64))
    }

    fn serialize_u32(self, v: u32) -> JsResult<JsValue> {
        Ok(JsValue::from_number(v as f64))
    }

    fn serialize_u64(self, v: u64) -> JsResult<JsValue> {
        Ok(JsValue::from_number(v as f64))
    }

    fn serialize_f32(self, v: f32) -> JsResult<JsValue> {
        Ok(JsValue::from_number(v as f64))
    }

    fn serialize_f64(self, v: f64) -> JsResult<JsValue> {
        Ok(JsValue::from_number(v))
    }

    fn serialize_char(self, v: char) -> JsResult<JsValue> {
        let mut buffer = [0; size_of::<char>()];
        let s = v.encode_utf8(&mut buffer);
        Ok(JsValue::from_string(s))
    }

    fn serialize_str(self, v: &str) -> JsResult<JsValue> {
        Ok(JsValue::from_string(v))
    }

    fn serialize_bytes(self, v: &[u8]) -> JsResult<JsValue> {
        Ok(JsValue::from_bytes(v))
    }

    fn serialize_none(self) -> JsResult<JsValue> {
        Ok(JsValue::null())
    }

    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> JsResult<JsValue> {
        to_js(value)
    }

    fn serialize_unit(self) -> JsResult<JsValue> {
        Ok(JsValue::undefined())
    }

    fn serialize_unit_struct(self, name: &'static str) -> JsResult<JsValue> {
        to_js(name)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> JsResult<JsValue> {
        Ok(JsValue::from_string(variant))
    }

    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> JsResult<JsValue> {
        to_js(value)
    }

    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> JsResult<JsValue> {
        let obj = JsValue::new_object();
        let k = JsValue::from_string(variant);
        let v = to_js(value)?;
        obj.object_append(k, v);
        Ok(obj)
    }

    fn serialize_seq(self, _len: Option<usize>) -> JsResult<JsValue> {
        Ok(JsValue::new_array())
    }

    fn serialize_tuple(self, _len: usize) -> JsResult<JsValue> {
        Ok(JsValue::new_array())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> JsResult<JsValue> {
        Ok(JsValue::new_array())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> JsResult<JsTupleVariantSerializer> {
        let obj = JsValue::new_object();
        let k = JsValue::from_string(variant);
        let arr = JsValue::new_array();
        arr.object_append(k, arr);
        Ok(JsTupleVariantSerializer { parent: obj, child: arr })
    }

    fn serialize_map(self, len: Option<usize>) -> JsResult<JsMapSerializer> {
        let object = JsValue::new_object();
        Ok(JsMapSerializer { obj: object, next_key: None })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> JsResult<JsValue> {
        Ok(JsValue::new_object())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> JsResult<JsStructVariantSerializer> {
        let obj = JsValue::new_object();
        let k = JsValue::from_string(variant);
        let sub = JsValue::new_object();
        obj.object_append(k, sub);
        Ok(JsStructVariantSerializer { parent: obj, child: sub })
    }
}

impl SerializeSeq for JsValue {
    type Ok = Self;

    type Error = Self;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> JsResult<()> {
        let v = to_js(value)?;
        self.array_append(v);
        Ok(())
    }

    fn end(self) -> JsResult<JsValue> {
        Ok(self)
    }
}

impl SerializeTuple for JsValue {
    type Ok = Self;

    type Error = Self;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> JsResult<()> {
        let v = to_js(value)?;
        self.array_append(v);
        Ok(())
    }

    fn end(self) -> JsResult<JsValue> {
        Ok(self)
    }
}

impl SerializeTupleStruct for JsValue {
    type Ok = Self;

    type Error = Self;

    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> JsResult<()> {
        let v = to_js(value)?;
        self.array_append(v);
        Ok(())
    }

    fn end(self) -> JsResult<JsValue> {
        Ok(self)
    }
}

impl SerializeTupleVariant for JsTupleVariantSerializer {
    type Ok = JsValue;

    type Error = JsValue;

    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> JsResult<()> {
        let v = to_js(value)?;
        self.child.array_append(v);
        Ok(())
    }

    fn end(self) -> JsResult<JsValue> {
        Ok(self.parent)
    }
}

impl SerializeMap for JsMapSerializer {
    type Ok = JsValue;

    type Error = JsValue;

    fn serialize_key<T: Serialize + ?Sized>(&mut self, key: &T) -> JsResult<()> {
        self.next_key = Some(to_js(key)?);
        Ok(())
    }

    fn serialize_value<T: Serialize + ?Sized>(&mut self, value: &T) -> JsResult<()> {
        let k = self.next_key.take().unwrap();
        let v = to_js(value)?;
        self.obj.object_append(k, v);
        Ok(())
    }

    fn end(self) -> JsResult<JsValue> {
        Ok(self.obj)
    }
}

impl SerializeStruct for JsValue {
    type Ok = Self;

    type Error = Self;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> JsResult<()> {
        let k = JsValue::from_string(key);
        let v = to_js(value);
        Ok(())
    }

    fn end(self) -> JsResult<JsValue> {
        Ok(self)
    }
}

impl SerializeStructVariant for JsStructVariantSerializer {
    type Ok = JsValue;

    type Error = JsValue;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> JsResult<()> {
        let k = JsValue::from_string(key);
        let v = to_js(value)?;
        self.child.object_append(k, v);
        Ok(())
    }

    fn end(self) -> JsResult<JsValue> {
        Ok(self.parent)
    }
}

/// Serializes a rust value into a js-owned one.
pub fn to_js<T: Serialize + ?Sized>(val: &T) -> JsResult<JsValue> {
    val.serialize(JsSerializer)
}
