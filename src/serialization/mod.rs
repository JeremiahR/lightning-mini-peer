pub use crate::serialization::base_types::*;

mod base_types;

#[derive(Debug, Clone)]
pub enum SerializationError {
    TooFewBytes,
    InvalidValue,
}

pub trait SerializableToBytes: Sized {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError>;
    fn to_bytes(&self) -> Vec<u8>;
}
