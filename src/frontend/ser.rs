use core::fmt::Display;

use serde::{Serializer, Serialize};
use serde::ser::{SerializeSeq, SerializeTuple, SerializeTupleStruct, SerializeTupleVariant, SerializeMap, SerializeStruct, SerializeStructVariant};

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
        todo!()
    }

    fn serialize_i8(self, v: i8) -> JsResult<JsValue> {
        todo!()
    }

    fn serialize_i16(self, v: i16) -> JsResult<JsValue> {
        todo!()
    }

    fn serialize_i32(self, v: i32) -> JsResult<JsValue> {
        todo!()
    }

    fn serialize_i64(self, v: i64) -> JsResult<JsValue> {
        todo!()
    }

    fn serialize_u8(self, v: u8) -> JsResult<JsValue> {
        todo!()
    }

    fn serialize_u16(self, v: u16) -> JsResult<JsValue> {
        todo!()
    }

    fn serialize_u32(self, v: u32) -> JsResult<JsValue> {
        todo!()
    }

    fn serialize_u64(self, v: u64) -> JsResult<JsValue> {
        todo!()
    }

    fn serialize_f32(self, v: f32) -> JsResult<JsValue> {
        todo!()
    }

    fn serialize_f64(self, v: f64) -> JsResult<JsValue> {
        todo!()
    }

    fn serialize_char(self, v: char) -> JsResult<JsValue> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> JsResult<JsValue> {
        todo!()
    }

    fn serialize_bytes(self, v: &[u8]) -> JsResult<JsValue> {
        todo!()
    }

    fn serialize_none(self) -> JsResult<JsValue> {
        todo!()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> JsResult<JsValue>
    where
        T: serde::Serialize {
        todo!()
    }

    fn serialize_unit(self) -> JsResult<JsValue> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> JsResult<JsValue> {
        todo!()
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

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, JsValue> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, JsValue> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, JsValue> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, JsValue> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, JsValue> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, JsValue> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, JsValue> {
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
