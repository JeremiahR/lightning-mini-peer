use crate::message_types::MessageType;
use std::collections::HashMap;
use strum::IntoEnumIterator;

#[derive(Debug, Clone)]
pub enum SerializationError {
    TooFewBytes,
}

pub trait BytesSerializable: Sized {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError>;
    fn to_bytes(&self) -> Vec<u8>;
}

#[derive(Debug, Clone)]
pub struct MessageTypeStruct {
    pub id: u16,
    pub name: String,
}

impl MessageTypeStruct {
    // it's lazy to do this every time, but it can be optimized later
    fn enum_name_lookup() -> HashMap<i32, String> {
        let mut map = HashMap::new();
        for variant in MessageType::iter() {
            let name: &str = variant.clone().into();
            let name = name.to_lowercase();
            map.insert(variant.clone() as i32, name);
        }
        map
    }
}

impl BytesSerializable for MessageTypeStruct {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 2 {
            return Err(SerializationError::TooFewBytes);
        }
        let id = u16::from_be_bytes([data[0], data[1]]);
        let lookup = Self::enum_name_lookup();
        let name = match lookup.get(&(id as i32)) {
            Some(name) => name.to_string(),
            None => "unknown".to_string(),
        };
        Ok((MessageTypeStruct { id, name }, &data[2..]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.id.to_be_bytes().to_vec()
    }
}

#[derive(Debug)]
pub struct U16SizedBytesStruct {
    num_bytes: u16,
    data: Vec<u8>,
}

impl BytesSerializable for U16SizedBytesStruct {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 2 {
            return Err(SerializationError::TooFewBytes);
        }
        let num_bytes = u16::from_be_bytes([data[0], data[1]]);
        let our_data = data[2..2 + num_bytes as usize].to_vec();
        Ok((
            U16SizedBytesStruct {
                num_bytes,
                data: our_data,
            },
            &data[2 as usize + num_bytes as usize..],
        ))
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.num_bytes.to_be_bytes().to_vec();
        bytes.extend(self.data.clone());
        bytes
    }
}

#[derive(Debug)]
pub struct ByteElement {
    value: u8,
}

impl BytesSerializable for ByteElement {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 1 {
            return Err(SerializationError::TooFewBytes);
        }
        Ok((ByteElement { value: data[0] }, &data[1..]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.value]
    }
}

#[derive(Debug)]
pub struct RGBColorElement {
    bytes: [u8; 3],
}

impl BytesSerializable for RGBColorElement {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 3 {
            return Err(SerializationError::TooFewBytes);
        }
        Ok((
            RGBColorElement {
                bytes: data[..3].try_into().unwrap(),
            },
            &data[3..],
        ))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }
}

#[derive(Debug)]
pub struct U16SerializedElement {
    value: u16,
}

impl BytesSerializable for U16SerializedElement {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 2 {
            return Err(SerializationError::TooFewBytes);
        }
        let value = u16::from_be_bytes([data[0], data[1]]);
        Ok((U16SerializedElement { value }, &data[2..]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}

#[derive(Debug)]
pub struct U32SerializedElement {
    value: u32,
}

impl BytesSerializable for U32SerializedElement {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 4 {
            return Err(SerializationError::TooFewBytes);
        }
        let value = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        Ok((U32SerializedElement { value }, &data[4..]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}

#[derive(Debug)]
pub struct Bytes64Element {
    data: [u8; 64],
}

impl BytesSerializable for Bytes64Element {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 64 {
            return Err(SerializationError::TooFewBytes);
        }
        let mut bytes = [0u8; 64];
        bytes.copy_from_slice(&data[..64]);
        Ok((Bytes64Element { data: bytes }, &data[64..]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}

pub type SignatureElement = Bytes64Element;

#[derive(Debug)]
pub struct Bytes32Element {
    data: [u8; 32],
}

impl BytesSerializable for Bytes32Element {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 32 {
            return Err(SerializationError::TooFewBytes);
        }
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&data[..32]);
        Ok((Bytes32Element { data: bytes }, &data[32..]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}

pub type ChainHashElement = Bytes32Element;
pub type NodeAliasElement = Bytes32Element;

#[derive(Debug)]
pub struct Bytes33Element {
    data: [u8; 33],
}

impl BytesSerializable for Bytes33Element {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 33 {
            return Err(SerializationError::TooFewBytes);
        }
        let mut bytes = [0u8; 33];
        bytes.copy_from_slice(&data[..33]);
        Ok((Bytes33Element { data: bytes }, &data[33..]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}

pub type PointElement = Bytes33Element;

#[derive(Debug)]
pub struct Bytes8Element {
    data: [u8; 8],
}

impl BytesSerializable for Bytes8Element {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 8 {
            return Err(SerializationError::TooFewBytes);
        }
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&data[..8]);
        Ok((Bytes8Element { data: bytes }, &data[8..]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.data.to_vec()
    }
}

pub type ShortChannelIDElement = Bytes8Element;

#[derive(Debug)]
pub struct RemainderElement {
    data: Vec<u8>,
}

impl BytesSerializable for RemainderElement {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        Ok((
            RemainderElement {
                data: data.to_vec(),
            },
            &data[0..0],
        ))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.data.clone()
    }
}

pub type TLVStreamElement = RemainderElement;

#[derive(Debug)]
pub enum SerializedKind {
    MessageType,
    U16Element,
    U32Element,
    U16SizedBytes,
    TLVStream,
    Signature,
    ChainHash,
    ShortChannelID,
    Point,
    Byte,
    RGBColor,
    NodeAlias,
}

#[derive(Debug)]
pub enum SerializedTypeContainer {
    MessageType(MessageTypeStruct),
    U16Element(U16SerializedElement),
    U32Element(U32SerializedElement),
    U16SizedBytes(U16SizedBytesStruct),
    TLVStream(TLVStreamElement),
    ChainHash(ChainHashElement),
    ShortChannelID(ShortChannelIDElement),
    Point(PointElement),
    Byte(ByteElement),
    Signature(SignatureElement),
    RGBColor(RGBColorElement),
    NodeAlias(NodeAliasElement),
}

impl SerializedTypeContainer {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            SerializedTypeContainer::MessageType(element) => element.to_bytes(),
            SerializedTypeContainer::U16SizedBytes(element) => element.to_bytes(),
            SerializedTypeContainer::TLVStream(element) => element.to_bytes(),
            SerializedTypeContainer::U16Element(element) => element.to_bytes(),
            SerializedTypeContainer::ChainHash(element) => element.to_bytes(),
            SerializedTypeContainer::ShortChannelID(element) => element.to_bytes(),
            SerializedTypeContainer::Point(element) => element.to_bytes(),
            SerializedTypeContainer::Signature(element) => element.to_bytes(),
            SerializedTypeContainer::U32Element(element) => element.to_bytes(),
            SerializedTypeContainer::RGBColor(element) => element.to_bytes(),
            SerializedTypeContainer::NodeAlias(element) => element.to_bytes(),
            SerializedTypeContainer::Byte(element) => element.to_bytes(),
        }
    }
}
