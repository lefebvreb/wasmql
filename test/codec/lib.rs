#![no_std]

use lazy_static::lazy_static;
use wasmql::prelude::*;

#[wasmql::codec]
pub trait TestCodec {
    fn echo(self, data: EchoData) -> EchoData;
}

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
    // pub float: f32,
    // pub string: String,
    //pub record: Record,
    //pub new_type: NewType,
}

lazy_static! {
    pub static ref DATA: EchoData = EchoData {
        int: -8265628,
    };
}
