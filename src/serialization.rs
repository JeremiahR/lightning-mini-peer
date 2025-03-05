use crate::message_types::MessageTypeEnum;
use std::collections::HashMap;
use strum::IntoEnumIterator;

enum SerializationError {
    Error,
}

pub trait Serializable: Sized {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), String>;
    fn to_bytes(&self) -> Vec<u8>;
}

#[derive(Debug, Clone)]
pub struct MessageTypeElement {
    pub id: u16,
    pub name: String,
}

impl MessageTypeElement {
    // it's lazy to do this every time, but it can be optimized later
    fn enum_name_lookup() -> HashMap<i32, String> {
        let mut map = HashMap::new();
        for variant in MessageTypeEnum::iter() {
            let name: &str = variant.clone().into();
            let name = name.to_lowercase();
            map.insert(variant.clone() as i32, name);
        }
        map
    }
}

impl Serializable for MessageTypeElement {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), String> {
        if data.len() < 2 {
            return Err("Not enough data to read a u16".to_string());
        }
        let id = u16::from_be_bytes([data[0], data[1]]);
        let lookup = Self::enum_name_lookup();
        let name = match lookup.get(&(id as i32)) {
            Some(name) => name.to_string(),
            None => "unknown".to_string(),
        };
        Ok((MessageTypeElement { id, name }, &data[2..]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.id.to_be_bytes().to_vec()
    }
}

#[derive(Debug)]
pub struct U16SizedBytesElement {
    num_bytes: u16,
    data: Vec<u8>,
}

impl Serializable for U16SizedBytesElement {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), String> {
        if data.len() < 2 {
            return Err("Not enough data to read a u16".to_string());
        }
        let num_bytes = u16::from_be_bytes([data[0], data[1]]);
        let our_data = data[2..2 + num_bytes as usize].to_vec();
        Ok((
            U16SizedBytesElement {
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

impl Serializable for ByteElement {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), String> {
        if data.len() < 1 {
            return Err("Not enough data to read a byte".to_string());
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

impl Serializable for RGBColorElement {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), String> {
        if data.len() < 1 {
            return Err("Not enough data to read a byte".to_string());
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

impl Serializable for U16SerializedElement {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), String> {
        if data.len() < 2 {
            return Err("Not enough data to read a u16".to_string());
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

impl Serializable for U32SerializedElement {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), String> {
        if data.len() < 4 {
            return Err("Not enough data to read a u16".to_string());
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

impl Serializable for Bytes64Element {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), String> {
        if data.len() < 64 {
            return Err("Not enough data to read a signature".to_string());
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

impl Serializable for Bytes32Element {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), String> {
        if data.len() < 32 {
            return Err("Not enough data to read a signature".to_string());
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

impl Serializable for Bytes33Element {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), String> {
        if data.len() < 33 {
            return Err("Not enough data to read a signature".to_string());
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

impl Serializable for Bytes8Element {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), String> {
        if data.len() < 8 {
            return Err("Not enough data to read a signature".to_string());
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

impl Serializable for RemainderElement {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), String> {
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
pub enum SerializableTypes {
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
pub enum SerializableElement {
    MessageType(MessageTypeElement),
    U16SizedBytes(U16SizedBytesElement),
    TLVStream(TLVStreamElement),
    U16Element(U16SerializedElement),
    U32Element(U32SerializedElement),
    ChainHash(ChainHashElement),
    ShortChannelID(ShortChannelIDElement),
    Point(PointElement),
    Byte(ByteElement),
    Signature(SignatureElement),
    RGBColor(RGBColorElement),
    NodeAlias(NodeAliasElement),
}

impl SerializableElement {
    pub fn from_bytes(key: SerializableElement, data: &[u8]) -> Result<(Self, &[u8]), String> {
        match key {
            SerializableElement::MessageType(_) => {
                let (res, data) = MessageTypeElement::from_bytes(data).unwrap();
                Ok((SerializableElement::MessageType(res), data))
            }
            SerializableElement::U16SizedBytes(_) => {
                let (res, data) = U16SizedBytesElement::from_bytes(data).unwrap();
                Ok((SerializableElement::U16SizedBytes(res), data))
            }
            SerializableElement::U16Element(_) => {
                let (res, data) = U16SerializedElement::from_bytes(data).unwrap();
                Ok((SerializableElement::U16Element(res), data))
            }
            SerializableElement::U32Element(_) => {
                let (res, data) = U32SerializedElement::from_bytes(data).unwrap();
                Ok((SerializableElement::U32Element(res), data))
            }
            SerializableElement::TLVStream(_) => {
                let (res, data) = TLVStreamElement::from_bytes(data).unwrap();
                Ok((SerializableElement::TLVStream(res), data))
            }
            SerializableElement::ChainHash(_) => {
                let (res, data) = ChainHashElement::from_bytes(data).unwrap();
                Ok((SerializableElement::ChainHash(res), data))
            }
            SerializableElement::ShortChannelID(_) => {
                let (res, data) = ShortChannelIDElement::from_bytes(data).unwrap();
                Ok((SerializableElement::ShortChannelID(res), data))
            }
            SerializableElement::Point(_) => {
                let (res, data) = PointElement::from_bytes(data).unwrap();
                Ok((SerializableElement::Point(res), data))
            }
            SerializableElement::Signature(_) => {
                let (res, data) = SignatureElement::from_bytes(data).unwrap();
                Ok((SerializableElement::Signature(res), data))
            }
            SerializableElement::RGBColor(_) => {
                let (res, data) = RGBColorElement::from_bytes(data).unwrap();
                Ok((SerializableElement::RGBColor(res), data))
            }
            SerializableElement::NodeAlias(_) => {
                let (res, data) = NodeAliasElement::from_bytes(data).unwrap();
                Ok((SerializableElement::NodeAlias(res), data))
            }
            SerializableElement::Byte(_) => {
                let (res, data) = ByteElement::from_bytes(data).unwrap();
                Ok((SerializableElement::Byte(res), data))
            }
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            SerializableElement::MessageType(element) => element.to_bytes(),
            SerializableElement::U16SizedBytes(element) => element.to_bytes(),
            SerializableElement::TLVStream(element) => element.to_bytes(),
            SerializableElement::U16Element(element) => element.to_bytes(),
            SerializableElement::ChainHash(element) => element.to_bytes(),
            SerializableElement::ShortChannelID(element) => element.to_bytes(),
            SerializableElement::Point(element) => element.to_bytes(),
            SerializableElement::Signature(element) => element.to_bytes(),
            SerializableElement::U32Element(element) => element.to_bytes(),
            SerializableElement::RGBColor(element) => element.to_bytes(),
            SerializableElement::NodeAlias(element) => element.to_bytes(),
            SerializableElement::Byte(element) => element.to_bytes(),
        }
    }
}
