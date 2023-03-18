use alloc::vec::Vec;
use serde::{Deserialize, Serialize};
use postcard::Result;

pub trait CodecExt {
    fn handle(bytes: &[u8]) -> Vec<u8>;
}

pub fn decode<'a, T: Deserialize<'a>>(bytes: &'a [u8]) -> Result<T> {
    postcard::from_bytes(bytes)
}

pub fn encode<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    postcard::to_allocvec(value)
}
