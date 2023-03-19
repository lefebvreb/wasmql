use postcard::Result;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Eq, PartialEq, Debug, Error)]
pub enum Error<E> {
    #[error("codec error: {0}")]
    Postcard(postcard::Error),
    #[error("missing discriminant")]
    MissingDiscriminant,
    #[error("handler error: {0}")]
    Handler(E),
}

impl<E> From<postcard::Error> for Error<E> {
    fn from(err: postcard::Error) -> Self {
        Self::Postcard(err)
    }
}

pub fn decode<'a, T: Deserialize<'a>>(bytes: &'a [u8]) -> Result<T> {
    postcard::from_bytes(bytes)
}

pub fn encode<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    postcard::to_allocvec(value)
}
