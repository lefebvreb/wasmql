#![no_std] // Greatly reduce .wasm binary size

use wasmql::prelude::*; // Use String

#[wasmql::codec]
pub trait MyCodec { // Our WasmQL codec API
    fn greet(self, name: String) -> String;
}
