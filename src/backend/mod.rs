use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

pub fn discriminant(bytes: &[u8]) -> Result<u16> {
    match bytes {
        &[.., lo, hi] => Ok(u16::from_le_bytes([lo, hi])),
        _ => Err(Error::Discriminant),
    }
}

pub fn decode<'a, T: Deserialize<'a>>(bytes: &'a [u8]) -> Result<T> {
    Ok(postcard::from_bytes(bytes)?)
}

pub fn encode<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    Ok(postcard::to_allocvec(value)?)
}
