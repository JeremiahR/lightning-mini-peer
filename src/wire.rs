use crate::messages::MessageType;

#[derive(Debug, Clone)]
pub enum SerializationError {
    TooFewBytes,
}

pub trait BytesSerializable: Sized {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError>;
    fn to_bytes(&self) -> Vec<u8>;
}

#[derive(Debug, Clone)]
pub struct MessageTypeWire {
    pub id: u16,
}

impl MessageTypeWire {
    pub fn new(mtype: MessageType) -> Self {
        MessageTypeWire { id: mtype.as_u16() }
    }
}

impl BytesSerializable for MessageTypeWire {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 2 {
            return Err(SerializationError::TooFewBytes);
        }
        let id = u16::from_be_bytes([data[0], data[1]]);
        Ok((MessageTypeWire { id }, &data[2..]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.id.to_be_bytes().to_vec()
    }
}

#[derive(Debug)]
pub struct U16SizedBytesWire {
    num_bytes: u16,
    pub value: Vec<u8>,
}

impl U16SizedBytesWire {
    pub fn new(data: Vec<u8>) -> Self {
        U16SizedBytesWire {
            num_bytes: data.len() as u16,
            value: data,
        }
    }
}

impl BytesSerializable for U16SizedBytesWire {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 2 {
            return Err(SerializationError::TooFewBytes);
        }
        let num_bytes = u16::from_be_bytes([data[0], data[1]]);
        let our_data = data[2..2 + num_bytes as usize].to_vec();
        Ok((
            U16SizedBytesWire {
                num_bytes,
                value: our_data,
            },
            &data[2 as usize + num_bytes as usize..],
        ))
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.num_bytes.to_be_bytes().to_vec();
        bytes.extend(self.value.clone());
        bytes
    }
}

#[derive(Debug)]
pub struct SingleByteWire {
    pub value: u8,
}

impl SingleByteWire {
    pub fn new(value: u8) -> Self {
        SingleByteWire { value }
    }
}

impl BytesSerializable for SingleByteWire {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 1 {
            return Err(SerializationError::TooFewBytes);
        }
        Ok((SingleByteWire { value: data[0] }, &data[1..]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.value]
    }
}

#[derive(Debug)]
pub struct RGBColorWire {
    bytes: [u8; 3],
}

impl BytesSerializable for RGBColorWire {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 3 {
            return Err(SerializationError::TooFewBytes);
        }
        Ok((
            RGBColorWire {
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
pub struct U16IntWire {
    pub value: u16,
}

impl U16IntWire {
    pub fn new(value: u16) -> Self {
        U16IntWire { value }
    }
}

impl BytesSerializable for U16IntWire {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 2 {
            return Err(SerializationError::TooFewBytes);
        }
        let value = u16::from_be_bytes([data[0], data[1]]);
        Ok((U16IntWire { value }, &data[2..]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}

#[derive(Debug)]
pub struct U32IntWire {
    pub value: u32,
}

impl U32IntWire {
    pub fn new(value: u32) -> Self {
        U32IntWire { value }
    }
}

impl BytesSerializable for U32IntWire {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 4 {
            return Err(SerializationError::TooFewBytes);
        }
        let value = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        Ok((U32IntWire { value }, &data[4..]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}

#[derive(Debug)]
pub struct Wire64Bytes {
    pub value: [u8; 64],
}

impl Wire64Bytes {
    pub fn new(data: [u8; 64]) -> Self {
        Wire64Bytes { value: data }
    }
}

impl BytesSerializable for Wire64Bytes {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 64 {
            return Err(SerializationError::TooFewBytes);
        }
        let mut bytes = [0u8; 64];
        bytes.copy_from_slice(&data[..64]);
        Ok((Wire64Bytes { value: bytes }, &data[64..]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_vec()
    }
}

#[derive(Debug)]
pub struct Wire32Bytes {
    pub value: [u8; 32],
}

impl Wire32Bytes {
    pub fn new(data: [u8; 32]) -> Self {
        Wire32Bytes { value: data }
    }
}

impl BytesSerializable for Wire32Bytes {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 32 {
            return Err(SerializationError::TooFewBytes);
        }
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&data[..32]);
        Ok((Wire32Bytes { value: bytes }, &data[32..]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_vec()
    }
}

#[derive(Debug)]
pub struct Wire33Bytes {
    pub value: [u8; 33],
}

impl Wire33Bytes {
    pub fn new(data: [u8; 33]) -> Self {
        Wire33Bytes { value: data }
    }
}

impl BytesSerializable for Wire33Bytes {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 33 {
            return Err(SerializationError::TooFewBytes);
        }
        let mut bytes = [0u8; 33];
        bytes.copy_from_slice(&data[..33]);
        Ok((Wire33Bytes { value: bytes }, &data[33..]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_vec()
    }
}

#[derive(Debug)]
pub struct Bytes8Element {
    pub value: [u8; 8],
}

impl Bytes8Element {
    pub fn new(data: [u8; 8]) -> Self {
        Bytes8Element { value: data }
    }
}

impl BytesSerializable for Bytes8Element {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 8 {
            return Err(SerializationError::TooFewBytes);
        }
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(&data[..8]);
        Ok((Bytes8Element { value: bytes }, &data[8..]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_vec()
    }
}

#[derive(Debug)]
pub struct Bytes3Element {
    pub value: [u8; 3],
}

impl Bytes3Element {
    pub fn new(data: [u8; 3]) -> Self {
        Bytes3Element { value: data }
    }
}

impl BytesSerializable for Bytes3Element {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 3 {
            return Err(SerializationError::TooFewBytes);
        }
        let mut bytes = [0u8; 3];
        bytes.copy_from_slice(&data[..3]);
        Ok((Bytes3Element { value: bytes }, &data[3..]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_vec()
    }
}

#[derive(Debug)]
pub struct RemainderTypeWire {
    pub value: Vec<u8>,
}

impl RemainderTypeWire {
    pub fn new(data: Vec<u8>) -> Self {
        RemainderTypeWire { value: data }
    }
}

impl BytesSerializable for RemainderTypeWire {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        Ok((
            RemainderTypeWire {
                value: data.to_vec(),
            },
            &data[0..0],
        ))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.clone()
    }
}

pub type IgnoredStruct = U16SizedBytesWire;
pub type NumPongBytesStruct = U16IntWire;
pub type GlobalFeaturesStruct = U16SizedBytesWire;
pub type LocalFeaturesStruct = U16SizedBytesWire;
pub type TimestampElement = U32IntWire;
pub type TimestampRangeElement = U32IntWire;
pub type FeaturesStruct = U16SizedBytesWire;
pub type TLVStreamElement = RemainderTypeWire;
pub type ShortChannelIDElement = Bytes8Element;
pub type SignatureElement = Wire64Bytes;
pub type ChainHashElement = Wire32Bytes;
#[allow(dead_code)]
pub type NodeAliasElement = Wire32Bytes;
pub type PointElementWire = Wire33Bytes;
