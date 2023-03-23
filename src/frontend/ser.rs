//! Serializing from rust type to js value.

use core::mem::size_of;

use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};
use serde::{Serialize, Serializer};

use super::js::JsValue;
use super::panic::{Result, Throw};

struct JsSerializer;

struct Tuple {
    parent: JsValue,
    child: JsValue,
}

struct Map {
    obj: JsValue,
    next_key: Option<JsValue>,
}

struct StructVariant {
    parent: JsValue,
    child: JsValue,
}

impl Serializer for JsSerializer {
    type Ok = JsValue;

    type Error = Throw;

    type SerializeSeq = JsValue;

    type SerializeTuple = JsValue;

    type SerializeTupleStruct = JsValue;

    type SerializeTupleVariant = Tuple;

    type SerializeMap = Map;

    type SerializeStruct = JsValue;

    type SerializeStructVariant = StructVariant;

    fn serialize_bool(self, v: bool) -> Result<JsValue> {
        Ok(JsValue::from_bool(v))
    }

    fn serialize_i8(self, v: i8) -> Result<JsValue> {
        Ok(JsValue::from_number(v as f64))
    }

    fn serialize_i16(self, v: i16) -> Result<JsValue> {
        Ok(JsValue::from_number(v as f64))
    }

    fn serialize_i32(self, v: i32) -> Result<JsValue> {
        Ok(JsValue::from_number(v as f64))
    }

    fn serialize_i64(self, v: i64) -> Result<JsValue> {
        Ok(JsValue::from_number(v as f64))
    }

    fn serialize_u8(self, v: u8) -> Result<JsValue> {
        Ok(JsValue::from_number(v as f64))
    }

    fn serialize_u16(self, v: u16) -> Result<JsValue> {
        Ok(JsValue::from_number(v as f64))
    }

    fn serialize_u32(self, v: u32) -> Result<JsValue> {
        Ok(JsValue::from_number(v as f64))
    }

    fn serialize_u64(self, v: u64) -> Result<JsValue> {
        Ok(JsValue::from_number(v as f64))
    }

    fn serialize_f32(self, v: f32) -> Result<JsValue> {
        Ok(JsValue::from_number(v as f64))
    }

    fn serialize_f64(self, v: f64) -> Result<JsValue> {
        Ok(JsValue::from_number(v))
    }

    fn serialize_char(self, v: char) -> Result<JsValue> {
        let mut buffer = [0; size_of::<char>()];
        let s = v.encode_utf8(&mut buffer);
        Ok(JsValue::from_string(s))
    }

    fn serialize_str(self, v: &str) -> Result<JsValue> {
        Ok(JsValue::from_string(v))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<JsValue> {
        Ok(JsValue::from_bytes(v))
    }

    fn serialize_none(self) -> Result<JsValue> {
        Ok(JsValue::null())
    }

    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<JsValue> {
        to_js(value)
    }

    fn serialize_unit(self) -> Result<JsValue> {
        Ok(JsValue::undefined())
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<JsValue> {
        to_js(name)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<JsValue> {
        Ok(JsValue::from_string(variant))
    }

    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<JsValue> {
        to_js(value)
    }

    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<JsValue> {
        let obj = JsValue::new_object();
        let k = JsValue::from_string(variant);
        let v = to_js(value)?;
        obj.object_append(k, v);
        Ok(obj)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<JsValue> {
        Ok(JsValue::new_array())
    }

    fn serialize_tuple(self, _len: usize) -> Result<JsValue> {
        Ok(JsValue::new_array())
    }

    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<JsValue> {
        Ok(JsValue::new_array())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Tuple> {
        let obj = JsValue::new_object();
        let k = JsValue::from_string(variant);
        let arr = JsValue::new_array();
        arr.object_append(k, arr);
        Ok(Tuple {
            parent: obj,
            child: arr,
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Map> {
        let object = JsValue::new_object();
        Ok(Map {
            obj: object,
            next_key: None,
        })
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<JsValue> {
        Ok(JsValue::new_object())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<StructVariant> {
        let obj = JsValue::new_object();
        let k = JsValue::from_string(variant);
        let sub = JsValue::new_object();
        obj.object_append(k, sub);
        Ok(StructVariant {
            parent: obj,
            child: sub,
        })
    }
}

impl SerializeSeq for JsValue {
    type Ok = Self;

    type Error = Throw;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        let v = to_js(value)?;
        self.array_append(v);
        Ok(())
    }

    fn end(self) -> Result<JsValue> {
        Ok(self)
    }
}

impl SerializeTuple for JsValue {
    type Ok = Self;

    type Error = Throw;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        let v = to_js(value)?;
        self.array_append(v);
        Ok(())
    }

    fn end(self) -> Result<JsValue> {
        Ok(self)
    }
}

impl SerializeTupleStruct for JsValue {
    type Ok = Self;

    type Error = Throw;

    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        let v = to_js(value)?;
        self.array_append(v);
        Ok(())
    }

    fn end(self) -> Result<JsValue> {
        Ok(self)
    }
}

impl SerializeTupleVariant for Tuple {
    type Ok = JsValue;

    type Error = Throw;

    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        let v = to_js(value)?;
        self.child.array_append(v);
        Ok(())
    }

    fn end(self) -> Result<JsValue> {
        Ok(self.parent)
    }
}

impl SerializeMap for Map {
    type Ok = JsValue;

    type Error = Throw;

    fn serialize_key<T: Serialize + ?Sized>(&mut self, key: &T) -> Result<()> {
        self.next_key = Some(to_js(key)?);
        Ok(())
    }

    fn serialize_value<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        let k = self.next_key.take().unwrap();
        let v = to_js(value)?;
        self.obj.object_append(k, v);
        Ok(())
    }

    fn end(self) -> Result<JsValue> {
        Ok(self.obj)
    }
}

impl SerializeStruct for JsValue {
    type Ok = Self;

    type Error = Throw;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()> {
        let k = JsValue::from_string(key);
        let v = to_js(value)?;
        self.object_append(k, v);
        Ok(())
    }

    fn end(self) -> Result<JsValue> {
        Ok(self)
    }
}

impl SerializeStructVariant for StructVariant {
    type Ok = JsValue;

    type Error = Throw;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()> {
        let k = JsValue::from_string(key);
        let v = to_js(value)?;
        self.child.object_append(k, v);
        Ok(())
    }

    fn end(self) -> Result<JsValue> {
        Ok(self.parent)
    }
}

/// Serializes a rust value into a js-owned one.
pub fn to_js<T: Serialize + ?Sized>(val: &T) -> Result<JsValue> {
    val.serialize(JsSerializer)
}
