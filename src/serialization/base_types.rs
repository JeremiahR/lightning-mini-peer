use std::fmt;

use crate::messages::MessageType;
use crate::serialization::{SerializableToBytes, SerializationError};

#[derive(Debug, Clone)]
pub struct MessageTypeElement {
    pub id: u16,
}

impl MessageTypeElement {
    pub fn new(mtype: MessageType) -> Self {
        MessageTypeElement { id: mtype.as_u16() }
    }
}

impl SerializableToBytes for MessageTypeElement {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 2 {
            return Err(SerializationError::TooFewBytes);
        }
        let id = u16::from_be_bytes([data[0], data[1]]);
        Ok((MessageTypeElement { id }, &data[2..]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.id.to_be_bytes().to_vec()
    }
}

#[derive(Debug, Clone)]
pub struct WireU16SizedBytes {
    num_bytes: u16,
    pub value: Vec<u8>,
}

impl WireU16SizedBytes {
    pub fn new(data: Vec<u8>) -> Self {
        WireU16SizedBytes {
            num_bytes: data.len() as u16,
            value: data,
        }
    }
}

impl SerializableToBytes for WireU16SizedBytes {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 2 {
            return Err(SerializationError::TooFewBytes);
        }
        let num_bytes = u16::from_be_bytes([data[0], data[1]]);
        let our_data = data[2..2 + num_bytes as usize].to_vec();
        Ok((
            WireU16SizedBytes {
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

#[derive(Clone)]
pub struct IgnoredBytesElement {
    pub value: WireU16SizedBytes,
}

impl IgnoredBytesElement {
    pub fn new(data: Vec<u8>) -> Self {
        IgnoredBytesElement {
            value: WireU16SizedBytes::new(data),
        }
    }
}

impl fmt::Debug for IgnoredBytesElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(ignored {} bytes)", self.value.num_bytes)
    }
}

impl SerializableToBytes for IgnoredBytesElement {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (value, rest) = WireU16SizedBytes::from_bytes(data).unwrap();
        Ok((IgnoredBytesElement { value }, rest))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_bytes()
    }
}

#[derive(Debug)]
pub struct NodeAddressesElement {
    pub ipv4_addresses: Vec<[u8; 6]>,
    pub ipv6_addresses: Vec<[u8; 16]>,
    pub torv2_addresses: Vec<[u8; 12]>,
    pub torv3_addresses: Vec<[u8; 37]>,
    pub dns_hostname: Vec<u8>,
}

impl SerializableToBytes for NodeAddressesElement {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (wrapper_struct, rest) = WireU16SizedBytes::from_bytes(data).unwrap();
        let mut ipv4_addresses = Vec::new();
        let mut ipv6_addresses = Vec::new();
        let mut torv2_addresses = Vec::new();
        let mut torv3_addresses = Vec::new();
        let mut dns_hostname = Vec::new();
        let mut buf = wrapper_struct.value.clone();
        loop {
            if buf.is_empty() {
                break;
            }
            let single_byte = buf[0];
            buf = buf[1..].to_vec();
            let chomp_bytes = match single_byte {
                1 => {
                    ipv4_addresses.push(buf[..6].try_into().unwrap());
                    6
                }
                2 => {
                    ipv6_addresses.push(buf[..18].try_into().unwrap());
                    18
                }
                3 => {
                    torv2_addresses.push(buf[..12].try_into().unwrap());
                    12
                }
                4 => {
                    torv3_addresses.push(buf[..37].try_into().unwrap());
                    37
                }
                5 => {
                    dns_hostname.extend(&buf);
                    buf.len()
                } // for dns_hostname chomp the rest of the buffer
                _ => return Err(SerializationError::InvalidValue),
            };
            buf = buf[chomp_bytes..].to_vec();
        }
        Ok((
            NodeAddressesElement {
                ipv4_addresses,
                ipv6_addresses,
                torv2_addresses,
                torv3_addresses,
                dns_hostname,
            },
            rest,
        ))
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();
        for address in self.ipv4_addresses.iter() {
            buf.extend([1u8]);
            buf.extend(address);
        }
        for address in self.ipv6_addresses.iter() {
            buf.extend([2u8]);
            buf.extend(address);
        }
        for address in self.torv2_addresses.iter() {
            buf.extend([3u8]);
            buf.extend(address);
        }
        for address in self.torv3_addresses.iter() {
            buf.extend([4u8]);
            buf.extend(address);
        }
        if !self.dns_hostname.is_empty() {
            buf.extend([5u8]);
            buf.extend(self.dns_hostname.clone());
        }
        WireU16SizedBytes::new(buf).to_bytes()
    }
}

#[derive(Debug)]
pub struct Wire1Byte {
    pub value: u8,
}

impl Wire1Byte {
    pub fn new(value: u8) -> Self {
        Wire1Byte { value }
    }
}

impl SerializableToBytes for Wire1Byte {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 1 {
            return Err(SerializationError::TooFewBytes);
        }
        Ok((Wire1Byte { value: data[0] }, &data[1..]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![self.value]
    }
}

#[derive(Debug)]
pub struct RGBColorWire {
    bytes: [u8; 3],
}

impl SerializableToBytes for RGBColorWire {
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
pub struct WireU16Int {
    pub value: u16,
}

impl WireU16Int {
    pub fn new(value: u16) -> Self {
        WireU16Int { value }
    }
}

impl SerializableToBytes for WireU16Int {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 2 {
            return Err(SerializationError::TooFewBytes);
        }
        let value = u16::from_be_bytes([data[0], data[1]]);
        Ok((WireU16Int { value }, &data[2..]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}

#[derive(Debug)]
pub struct WireU32Int {
    pub value: u32,
}

impl WireU32Int {
    pub fn new(value: u32) -> Self {
        WireU32Int { value }
    }
}

impl SerializableToBytes for WireU32Int {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 4 {
            return Err(SerializationError::TooFewBytes);
        }
        let value = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
        Ok((WireU32Int { value }, &data[4..]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}

#[derive(Debug)]
pub struct WireU64Int {
    pub value: u64,
}

impl WireU64Int {
    pub fn new(value: u64) -> Self {
        WireU64Int { value }
    }
}

impl SerializableToBytes for WireU64Int {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 4 {
            return Err(SerializationError::TooFewBytes);
        }
        let value = u64::from_be_bytes([
            data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
        ]);
        Ok((WireU64Int { value }, &data[8..]))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_be_bytes().to_vec()
    }
}

fn decode_64_bytes(data: &[u8]) -> Result<([u8; 64], &[u8]), SerializationError> {
    if data.len() < 64 {
        return Err(SerializationError::TooFewBytes);
    }
    let mut bytes = [0u8; 64];
    bytes.copy_from_slice(&data[..64]);
    Ok((bytes, &data[64..]))
}

#[derive(Debug)]
pub struct Wire64Bytes {
    pub value: [u8; 64],
}

impl SerializableToBytes for Wire64Bytes {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (bytes, data) = decode_64_bytes(data)?;
        Ok((Wire64Bytes { value: bytes }, data))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_vec()
    }
}

pub struct SignatureElement {
    value: [u8; 64],
}

impl SerializableToBytes for SignatureElement {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (bytes, data) = decode_64_bytes(data)?;
        Ok((SignatureElement { value: bytes }, data))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_vec()
    }
}

impl fmt::Debug for SignatureElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.value))
    }
}

fn decode_32_bytes(data: &[u8]) -> Result<([u8; 32], &[u8]), SerializationError> {
    if data.len() < 32 {
        return Err(SerializationError::TooFewBytes);
    }
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&data[..32]);
    Ok((bytes, &data[32..]))
}

#[derive(Debug)]
pub struct Wire32Bytes {
    pub value: [u8; 32],
}

impl SerializableToBytes for Wire32Bytes {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (data, remainder) = decode_32_bytes(data).unwrap();
        Ok((Wire32Bytes { value: data }, remainder))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_vec()
    }
}

pub struct NodeAliasElement {
    pub value: Wire32Bytes,
}

impl fmt::Debug for NodeAliasElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // find the first byte that is zero
        let mut i = 0;
        while i < self.value.value.len() && self.value.value[i] == 0 {
            i += 1;
        }
        // cast the first i bytes to a string
        let alias = String::from_utf8_lossy(&self.value.value[..i]);
        write!(f, "\"{}\"", alias)
    }
}

impl SerializableToBytes for NodeAliasElement {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (data, remainder) = Wire32Bytes::from_bytes(data)?;
        Ok((NodeAliasElement { value: data }, remainder))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_bytes()
    }
}

#[derive(Clone)]
pub struct ChainHashElement {
    pub value: [u8; 32],
}

impl fmt::Debug for ChainHashElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.value))
    }
}

impl SerializableToBytes for ChainHashElement {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (data, remainder) = decode_32_bytes(data).unwrap();
        Ok((ChainHashElement { value: data }, remainder))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_vec()
    }
}

fn decode_33_bytes(data: &[u8]) -> Result<([u8; 33], &[u8]), SerializationError> {
    if data.len() < 33 {
        return Err(SerializationError::TooFewBytes);
    }
    let mut bytes = [0u8; 33];
    bytes.copy_from_slice(&data[..33]);
    Ok((bytes, &data[33..]))
}

#[derive(Debug)]
pub struct Wire33Bytes {
    pub value: [u8; 33],
}

impl SerializableToBytes for Wire33Bytes {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (bytes, remainder) = decode_33_bytes(data).unwrap();
        Ok((Wire33Bytes { value: bytes }, remainder))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_vec()
    }
}

pub struct PointElement {
    pub value: [u8; 33],
}

impl fmt::Debug for PointElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", hex::encode(&self.value))
    }
}

impl SerializableToBytes for PointElement {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (bytes, remainder) = decode_33_bytes(data).unwrap();
        Ok((PointElement { value: bytes }, remainder))
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.value.to_vec()
    }
}

#[derive(Debug)]
pub struct Bytes8Element {
    pub value: [u8; 8],
}

impl SerializableToBytes for Bytes8Element {
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
pub struct ShortChannelIDElement {
    pub block_height: u32,
    pub tx_index: u32,
    pub output_index: u16,
}

impl ShortChannelIDElement {}

impl SerializableToBytes for ShortChannelIDElement {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 8 {
            return Err(SerializationError::TooFewBytes);
        }
        let block_height = u32::from_be_bytes([0, data[0], data[1], data[2]]);
        let tx_index = u32::from_be_bytes([0, data[3], data[4], data[5]]);
        let output_index = u16::from_be_bytes([data[6], data[7]]);
        Ok((
            ShortChannelIDElement {
                block_height,
                tx_index,
                output_index,
            },
            &data[8..],
        ))
    }

    fn to_bytes(&self) -> Vec<u8> {
        let block_bytes = self.block_height.to_be_bytes();
        let tx_bytes = self.tx_index.to_be_bytes();
        let output_bytes = self.output_index.to_be_bytes();
        [
            block_bytes[1],
            block_bytes[2],
            block_bytes[3],
            tx_bytes[1],
            tx_bytes[2],
            tx_bytes[3],
            output_bytes[0],
            output_bytes[1],
        ]
        .to_vec()
    }
}

#[derive(Debug)]
pub struct Wire3Bytes {
    pub value: [u8; 3],
}

impl Wire3Bytes {
    pub fn new(data: [u8; 3]) -> Self {
        Wire3Bytes { value: data }
    }
}

impl SerializableToBytes for Wire3Bytes {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        if data.len() < 3 {
            return Err(SerializationError::TooFewBytes);
        }
        let mut bytes = [0u8; 3];
        bytes.copy_from_slice(&data[..3]);
        Ok((Wire3Bytes { value: bytes }, &data[3..]))
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

impl SerializableToBytes for RemainderTypeWire {
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

pub type NumPongBytesElement = WireU16Int;
pub type GlobalFeaturesElement = WireU16SizedBytes;
pub type LocalFeaturesStruct = WireU16SizedBytes;
pub type TimestampElement = WireU32Int;
pub type TimestampRangeElement = WireU32Int;
pub type FeaturesElement = WireU16SizedBytes;
pub type TLVStreamElement = RemainderTypeWire;
