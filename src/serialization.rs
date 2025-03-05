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
    data: u8,
}

impl Serializable for ByteElement {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), String> {
        if data.len() < 1 {
            return Err("Not enough data to read a byte".to_string());
        }
        Ok((ByteElement { data: data[0] }, &data[1..]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.data]
    }
}

#[derive(Debug)]
pub struct U16SerializedElement {
    data: u16,
}

impl Serializable for U16SerializedElement {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), String> {
        if data.len() < 2 {
            return Err("Not enough data to read a u16".to_string());
        }
        let num_bytes = u16::from_be_bytes([data[0], data[1]]);
        Ok((U16SerializedElement { data: num_bytes }, &data[2..]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.data.to_be_bytes().to_vec()
    }
}

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
    U16SizedBytes,
    TLVStream,
}

#[derive(Debug)]
pub enum SerializableElement {
    MessageType(MessageTypeElement),
    U16SizedBytes(U16SizedBytesElement),
    TLVStream(TLVStreamElement),
    U16Element(U16SerializedElement),
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
            SerializableElement::TLVStream(_) => {
                let (res, data) = TLVStreamElement::from_bytes(data).unwrap();
                Ok((SerializableElement::TLVStream(res), data))
            }
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            SerializableElement::MessageType(element) => element.to_bytes(),
            SerializableElement::U16SizedBytes(element) => element.to_bytes(),
            SerializableElement::TLVStream(element) => element.to_bytes(),
            SerializableElement::U16Element(element) => element.to_bytes(),
        }
    }
}
