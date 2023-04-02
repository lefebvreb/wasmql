#![no_std]

extern crate alloc;

use alloc::vec;
use lazy_static::lazy_static;
use wasmql::prelude::*;

#[wasmql::codec]
pub trait TestCodec {
    fn echo(self, data: EchoData) -> EchoData;
}

const UTF8_TEST: &str = "~𝘈Ḇ𝖢𝕯٤ḞԍНǏ𝙅ƘԸⲘ𝙉০Ρ𝗤Ɍ𝓢ȚЦ𝒱Ѡ𝓧ƳȤѧᖯć𝗱ễ𝑓𝙜Ⴙ𝞲𝑗𝒌ļṃŉо𝞎𝒒ᵲꜱ𝙩ừ𝗏ŵ𝒙𝒚ź1234567890!@#$%^&*()-_=+[{]};:'\",<.>/?";

#[wasmql::data]
#[derive(PartialEq, Debug)]
pub struct NewType(pub u64);

#[wasmql::data]
#[derive(PartialEq, Debug)]
pub struct Record {
    k: String,
    v: i8,
}

#[wasmql::data]
#[derive(PartialEq, Debug)]
pub enum Enum {
    UnitVariant,
    NewTypeVariant(String),
    TupleVariant(f32, String, f64),
    StructVariant {
        a: i8,
        b: i8,
        c: i8,
    },
}

#[wasmql::data]
#[derive(PartialEq, Debug)]
pub struct EchoData {
    unit: (),
    boolean: bool,
    int: i32,
    float: f32,
    //chr: char,
    string: String,
    opt_some: Option<i8>,
    opt_none: Option<i8>,
    tup: (i32, String),
    arr: Vec<bool>,
    record: Record,
    new_type: NewType,
    enum_unit: Enum,
    enum_new_type: Enum,
    enum_tup: Enum,
    enum_struct: Enum,
}

lazy_static! {
    pub static ref TEST_DATA: EchoData = EchoData {
        unit: (),
        boolean: true,
        int: -8265628,
        float: 42.0,
        //chr: '$',
        string: UTF8_TEST.to_string(),
        opt_some: Some(-12),
        opt_none: None,
        tup: (7, "7".to_string()),
        arr: vec![true, false, true, false],
        record: Record { k: "key".to_string(), v: 42 },
        new_type: NewType(10000000),
        enum_unit: Enum::UnitVariant,
        enum_new_type: Enum::NewTypeVariant("john".to_string()),
        enum_tup: Enum::TupleVariant(1.0, "0.0".to_string(), -1.0),
        enum_struct: Enum::StructVariant { a: 1, b: 2, c: 3 },
    };
}
