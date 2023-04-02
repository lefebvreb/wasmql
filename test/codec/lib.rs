#![no_std]

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
    pub k: String,
    pub v: i8,
}

#[wasmql::data]
#[derive(PartialEq, Debug)]
pub struct EchoData {
    pub int: i32,
    pub float: f32,
    pub string: String,
    pub record: Record,
    pub new_type: NewType,
}

lazy_static! {
    pub static ref DATA: EchoData = EchoData {
        int: -8265628,
        float: 42.0,
        string: UTF8_TEST.to_string(),
        record: Record {
            k: "key".to_string(),
            v: 42,
        },
        new_type: NewType(10000000),
    };
}
