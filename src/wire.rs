use std::collections::HashMap;

use crate::message_types::MessageTypeEnum;
use crate::serialization::{
    ByteElement, ChainHashElement, MessageTypeElement, NodeAliasElement, PointElement,
    RGBColorElement, Serializable, SerializableElement, SerializableTypes, ShortChannelIDElement,
    SignatureElement, TLVStreamElement, U16SerializedElement, U16SizedBytesElement,
    U32SerializedElement,
};

#[derive(Debug)]
pub enum MessageDecodeError {
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum WireElement {
    MessageType,
    GlobalFeatures,
    LocalFeatures,
    TLVStream,
    NumPongBytes,
    Ignored,
    NodeSignature1,
    NodeSignature2,
    BitcoinSignature1,
    BitcoinSignature2,
    Timestamp,
    Features,
    ChainHash,
    ShortChannelID,
    NodeId,
    NodeId1,
    NodeId2,
    BitcoinKey1,
    BitcoinKey2,
    Signature,
    RGBColor,
    NodeAlias,
    Addresses,
    EncodedShortIds,
    QuerySortChannelIDsTLVS,
    FullInformation,
    FirstBlockNum,
    NumberOfBlocks,
    SyncComplete,
    QueryChannelRangeTLVs,
    ReplyChannelRangeTLVs,
    FirstTimestamp,
    TimestampRange,
}

impl WireElement {
    pub fn as_serializable(element: WireElement) -> SerializableTypes {
        match element {
            WireElement::MessageType => SerializableTypes::MessageType,
            WireElement::GlobalFeatures => SerializableTypes::U16SizedBytes,
            WireElement::LocalFeatures => SerializableTypes::U16SizedBytes,
            WireElement::TLVStream => SerializableTypes::TLVStream,
            WireElement::NumPongBytes => SerializableTypes::U16Element,
            WireElement::Ignored => SerializableTypes::U16SizedBytes,
            WireElement::Signature => SerializableTypes::Signature,
            WireElement::NodeSignature1 => SerializableTypes::Signature,
            WireElement::NodeSignature2 => SerializableTypes::Signature,
            WireElement::BitcoinSignature1 => SerializableTypes::Signature,
            WireElement::BitcoinSignature2 => SerializableTypes::Signature,
            WireElement::Features => SerializableTypes::U16SizedBytes,
            WireElement::ChainHash => SerializableTypes::ChainHash,
            WireElement::ShortChannelID => SerializableTypes::ShortChannelID,
            WireElement::NodeId => SerializableTypes::Point,
            WireElement::NodeId1 => SerializableTypes::Point,
            WireElement::NodeId2 => SerializableTypes::Point,
            WireElement::BitcoinKey1 => SerializableTypes::Point,
            WireElement::BitcoinKey2 => SerializableTypes::Point,
            WireElement::Timestamp => SerializableTypes::U32Element,
            WireElement::FirstTimestamp => SerializableTypes::U32Element,
            WireElement::TimestampRange => SerializableTypes::U32Element,
            WireElement::FirstBlockNum => SerializableTypes::U32Element,
            WireElement::NumberOfBlocks => SerializableTypes::U32Element,
            WireElement::RGBColor => SerializableTypes::RGBColor,
            WireElement::NodeAlias => SerializableTypes::NodeAlias,
            WireElement::Addresses => SerializableTypes::U16SizedBytes,
            WireElement::QuerySortChannelIDsTLVS => SerializableTypes::TLVStream,
            WireElement::QueryChannelRangeTLVs => SerializableTypes::TLVStream,
            WireElement::FullInformation => SerializableTypes::Byte,
            WireElement::SyncComplete => SerializableTypes::Byte,
            WireElement::EncodedShortIds => SerializableTypes::U16SizedBytes,
            WireElement::ReplyChannelRangeTLVs => SerializableTypes::TLVStream,
        }
    }
}

#[derive(Debug)]
pub struct WireFormatMessage {
    message_type: MessageTypeEnum,
    elements: HashMap<WireElement, SerializableElement>,
    element_order: Vec<WireElement>,
}

impl WireFormatMessage {
    pub fn get_structure(
        msg_type: u16,
    ) -> Result<(MessageTypeEnum, Vec<WireElement>), MessageDecodeError> {
        let type_enum = MessageTypeEnum::try_from(msg_type).unwrap();
        let wire_elements: Vec<WireElement> = match type_enum {
            MessageTypeEnum::Init => vec![
                WireElement::MessageType,
                WireElement::GlobalFeatures,
                WireElement::LocalFeatures,
                WireElement::TLVStream,
            ],
            MessageTypeEnum::Ping => vec![
                WireElement::MessageType,
                WireElement::NumPongBytes,
                WireElement::Ignored,
            ],
            MessageTypeEnum::Pong => vec![WireElement::MessageType, WireElement::Ignored],
            MessageTypeEnum::ChannelAnnouncement => vec![
                WireElement::NodeSignature1,
                WireElement::NodeSignature2,
                WireElement::BitcoinSignature1,
                WireElement::BitcoinSignature2,
                WireElement::Features,
                WireElement::ChainHash,
                WireElement::ShortChannelID,
                WireElement::NodeId1,
                WireElement::NodeId2,
                WireElement::BitcoinKey1,
                WireElement::BitcoinKey2,
            ],
            MessageTypeEnum::NodeAnnouncement => vec![
                WireElement::Signature,
                WireElement::Features,
                WireElement::Timestamp,
                WireElement::NodeId,
                WireElement::RGBColor,
                WireElement::NodeAlias,
                WireElement::Addresses,
            ],
            MessageTypeEnum::QueryShortChannelIds => vec![
                WireElement::ChainHash,
                WireElement::EncodedShortIds,
                WireElement::QuerySortChannelIDsTLVS,
            ],
            MessageTypeEnum::ReplyShortChannelIdsEnd => {
                vec![WireElement::ChainHash, WireElement::FullInformation]
            }
            MessageTypeEnum::QueryChannelRange => vec![
                WireElement::ChainHash,
                WireElement::FirstBlockNum,
                WireElement::NumberOfBlocks,
                WireElement::QueryChannelRangeTLVs,
            ],
            MessageTypeEnum::ReplyChannelRange => vec![
                WireElement::ChainHash,
                WireElement::FirstBlockNum,
                WireElement::NumberOfBlocks,
                WireElement::SyncComplete,
                WireElement::EncodedShortIds,
                WireElement::ReplyChannelRangeTLVs,
            ],
            MessageTypeEnum::GossipTimestampFilter => vec![
                WireElement::ChainHash,
                WireElement::FirstTimestamp,
                WireElement::TimestampRange,
            ],
            _ => vec![],
        };
        Ok((type_enum, wire_elements))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(WireFormatMessage, &[u8]), MessageDecodeError> {
        let (m, _) = MessageTypeElement::from_bytes(bytes).unwrap();
        let (message_type, wire_elements) = WireFormatMessage::get_structure(m.id).unwrap();
        let mut elements = HashMap::new();
        let mut element_order = Vec::new();
        let mut bytes = bytes;
        for wire_element in wire_elements {
            let (obj, rem_bytes) = match WireElement::as_serializable(wire_element.clone()) {
                SerializableTypes::MessageType => {
                    let (obj, bytes) = MessageTypeElement::from_bytes(bytes).unwrap();
                    (SerializableElement::MessageType(obj), bytes)
                }
                SerializableTypes::U16Element => {
                    let (obj, bytes) = U16SerializedElement::from_bytes(bytes).unwrap();
                    (SerializableElement::U16Element(obj), bytes)
                }
                SerializableTypes::U16SizedBytes => {
                    let (obj, bytes) = U16SizedBytesElement::from_bytes(bytes).unwrap();
                    (SerializableElement::U16SizedBytes(obj), bytes)
                }
                SerializableTypes::TLVStream => {
                    let (obj, bytes) = TLVStreamElement::from_bytes(bytes).unwrap();
                    (SerializableElement::TLVStream(obj), bytes)
                }
                SerializableTypes::Signature => {
                    let (obj, bytes) = SignatureElement::from_bytes(bytes).unwrap();
                    (SerializableElement::Signature(obj), bytes)
                }
                SerializableTypes::ChainHash => {
                    let (obj, bytes) = ChainHashElement::from_bytes(bytes).unwrap();
                    (SerializableElement::ChainHash(obj), bytes)
                }
                SerializableTypes::ShortChannelID => {
                    let (obj, bytes) = ShortChannelIDElement::from_bytes(bytes).unwrap();
                    (SerializableElement::ShortChannelID(obj), bytes)
                }
                SerializableTypes::Point => {
                    let (obj, bytes) = PointElement::from_bytes(bytes).unwrap();
                    (SerializableElement::Point(obj), bytes)
                }
                SerializableTypes::RGBColor => {
                    let (obj, bytes) = RGBColorElement::from_bytes(bytes).unwrap();
                    (SerializableElement::RGBColor(obj), bytes)
                }
                SerializableTypes::NodeAlias => {
                    let (obj, bytes) = NodeAliasElement::from_bytes(bytes).unwrap();
                    (SerializableElement::NodeAlias(obj), bytes)
                }
                SerializableTypes::U32Element => {
                    let (obj, bytes) = U32SerializedElement::from_bytes(bytes).unwrap();
                    (SerializableElement::U32Element(obj), bytes)
                }
                SerializableTypes::Byte => {
                    let (obj, bytes) = ByteElement::from_bytes(bytes).unwrap();
                    (SerializableElement::Byte(obj), bytes)
                }
            };
            bytes = rem_bytes;
            elements.insert(wire_element.clone(), obj);
            element_order.push(wire_element);
        }
        Ok((
            WireFormatMessage {
                message_type,
                elements,
                element_order,
            },
            bytes,
        ))
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for key in &self.element_order {
            let element = self.elements.get(key).unwrap();
            bytes.extend_from_slice(element.to_bytes().as_slice());
        }
        bytes
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_decode_init_message() {
        let initial_bytes = hex::decode("001000021100000708a0880a8a59a1012006226e46111a0b59caaf126043eb5bbf28c34f3a5e332a1fc7b2b73cf188910f2d7ef99482067a1b72fe9e411d37be8c").unwrap();
        let (msg, remainder) = WireFormatMessage::from_bytes(&initial_bytes).unwrap();
        assert_eq!(msg.message_type, MessageTypeEnum::Init);
        // check that "type" is contained in msg.elements
        assert!(msg.elements.contains_key(&WireElement::MessageType));
        assert!(msg.elements.contains_key(&WireElement::GlobalFeatures));
        assert!(msg.elements.contains_key(&WireElement::LocalFeatures));
        // check serialization
        assert_eq!([msg.to_bytes(), remainder.to_vec()].concat(), initial_bytes);
    }
}
