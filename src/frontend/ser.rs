//! Serializing from rust type to js value.

use core::fmt::Display;
use core::mem::size_of;

use serde::{Serializer, Serialize};
use serde::ser::{SerializeSeq, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant, SerializeMap, SerializeStruct, SerializeStructVariant};

use super::ffi;
use super::js::{JsValue, JsResult};

#[derive(Debug)]
pub struct JsSerializer;

impl Serializer for JsSerializer {
    type Ok = JsValue;

    type Error = JsValue;

    type SerializeSeq = JsValue;

    type SerializeTuple = JsValue;

    type SerializeTupleStruct = JsValue;

    type SerializeTupleVariant = JsValue;

    type SerializeMap = JsValue;

    type SerializeStruct = JsValue;

    type SerializeStructVariant = JsValue;

    fn serialize_bool(self, v: bool) -> JsResult<JsValue> {
        Ok(ffi::boolean(v))
    }

    fn serialize_i8(self, v: i8) -> JsResult<JsValue> {
        Ok(ffi::number(v as f64))
    }

    fn serialize_i16(self, v: i16) -> JsResult<JsValue> {
        Ok(ffi::number(v as f64))
    }

    fn serialize_i32(self, v: i32) -> JsResult<JsValue> {
        Ok(ffi::number(v as f64))
    }

    fn serialize_i64(self, v: i64) -> JsResult<JsValue> {
        Ok(ffi::number(v as f64))
    }

    fn serialize_u8(self, v: u8) -> JsResult<JsValue> {
        Ok(ffi::number(v as f64))
    }

    fn serialize_u16(self, v: u16) -> JsResult<JsValue> {
        Ok(ffi::number(v as f64))
    }

    fn serialize_u32(self, v: u32) -> JsResult<JsValue> {
        Ok(ffi::number(v as f64))
    }

    fn serialize_u64(self, v: u64) -> JsResult<JsValue> {
        Ok(ffi::number(v as f64))
    }

    fn serialize_f32(self, v: f32) -> JsResult<JsValue> {
        Ok(ffi::number(v as f64))
    }

    fn serialize_f64(self, v: f64) -> JsResult<JsValue> {
        Ok(ffi::number(v as f64))
    }

    fn serialize_char(self, v: char) -> JsResult<JsValue> {
        let mut buffer = [0; size_of::<char>()];
        let s = v.encode_utf8(&mut buffer);
        Ok(ffi::string(s))
    }

    fn serialize_str(self, v: &str) -> JsResult<JsValue> {
        Ok(ffi::string(v))
    }

    fn serialize_bytes(self, v: &[u8]) -> JsResult<JsValue> {
        Ok(ffi::bytes(v))
    }

    fn serialize_none(self) -> JsResult<JsValue> {
        Ok(ffi::null())
    }

    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> JsResult<JsValue> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> JsResult<JsValue> {
        Ok(ffi::undefined())
    }

    fn serialize_unit_struct(self, name: &'static str) -> JsResult<JsValue> {
        name.serialize(self)
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> JsResult<JsValue> {
        todo!()
    }

    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> JsResult<JsValue> {
        todo!()
    }

    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> JsResult<JsValue> {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> JsResult<JsValue> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> JsResult<JsValue> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> JsResult<JsValue> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> JsResult<JsValue> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> JsResult<JsValue> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> JsResult<JsValue> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> JsResult<JsValue> {
        todo!()
    }

    fn collect_str<T: Display + ?Sized>(self, value: &T) -> JsResult<JsValue> {
        todo!()
    }
}

impl SerializeSeq for JsValue {
    type Ok = Self;

    type Error = Self;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> JsResult<()> {
        todo!()
    }

    fn end(self) -> Result<Self, JsValue> {
        todo!()
    }
}

impl SerializeTuple for JsValue {
    type Ok = Self;

    type Error = Self;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> JsResult<()> {
        todo!()
    }

    fn end(self) -> Result<Self, JsValue> {
        todo!()
    }
}

impl SerializeTupleStruct for JsValue {
    type Ok = Self;

    type Error = Self;

    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> JsResult<()> {
        todo!()
    }

    fn end(self) -> Result<Self, JsValue> {
        todo!()
    }
}

impl SerializeTupleVariant for JsValue {
    type Ok = Self;

    type Error = Self;

    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> JsResult<()> {
        todo!()
    }

    fn end(self) -> Result<Self, JsValue> {
        todo!()
    }
}

impl SerializeMap for JsValue {
    type Ok = Self;

    type Error = Self;

    fn serialize_key<T: Serialize + ?Sized>(&mut self, key: &T) -> JsResult<()> {
        todo!()
    }

    fn serialize_value<T: Serialize + ?Sized>(&mut self, value: &T) -> JsResult<()> {
        todo!()
    }

    fn end(self) -> Result<Self, JsValue> {
        todo!()
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
        todo!()
    }

    fn end(self) -> Result<Self, JsValue> {
        todo!()
    }
}

impl SerializeStructVariant for JsValue {
    type Ok = Self;

    type Error = Self;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> JsResult<()> {
        todo!()
    }

    fn end(self) -> Result<Self, JsValue> {
        todo!()
    }
}
