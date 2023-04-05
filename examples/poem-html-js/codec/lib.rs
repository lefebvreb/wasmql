#![no_std]

use wasmql::prelude::*;

#[wasmql::codec]
pub trait MyCodec {
    fn greet(self, name: String) -> String;
}
