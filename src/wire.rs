use std::collections::HashMap;

use crate::message_types::MessageTypeEnum;
use crate::serialization::{
    ByteElement, ChainHashElement, MessageTypeElement, NodeAliasElement, PointElement,
    RGBColorElement, Serializable, SerializableType, SerializedContainer, ShortChannelIDElement,
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
    pub fn as_serializable(element: WireElement) -> SerializableType {
        match element {
            WireElement::MessageType => SerializableType::MessageType,
            WireElement::GlobalFeatures => SerializableType::U16SizedBytes,
            WireElement::LocalFeatures => SerializableType::U16SizedBytes,
            WireElement::TLVStream => SerializableType::TLVStream,
            WireElement::NumPongBytes => SerializableType::U16Element,
            WireElement::Ignored => SerializableType::U16SizedBytes,
            WireElement::Signature => SerializableType::Signature,
            WireElement::NodeSignature1 => SerializableType::Signature,
            WireElement::NodeSignature2 => SerializableType::Signature,
            WireElement::BitcoinSignature1 => SerializableType::Signature,
            WireElement::BitcoinSignature2 => SerializableType::Signature,
            WireElement::Features => SerializableType::U16SizedBytes,
            WireElement::ChainHash => SerializableType::ChainHash,
            WireElement::ShortChannelID => SerializableType::ShortChannelID,
            WireElement::NodeId => SerializableType::Point,
            WireElement::NodeId1 => SerializableType::Point,
            WireElement::NodeId2 => SerializableType::Point,
            WireElement::BitcoinKey1 => SerializableType::Point,
            WireElement::BitcoinKey2 => SerializableType::Point,
            WireElement::Timestamp => SerializableType::U32Element,
            WireElement::FirstTimestamp => SerializableType::U32Element,
            WireElement::TimestampRange => SerializableType::U32Element,
            WireElement::FirstBlockNum => SerializableType::U32Element,
            WireElement::NumberOfBlocks => SerializableType::U32Element,
            WireElement::RGBColor => SerializableType::RGBColor,
            WireElement::NodeAlias => SerializableType::NodeAlias,
            WireElement::Addresses => SerializableType::U16SizedBytes,
            WireElement::QuerySortChannelIDsTLVS => SerializableType::TLVStream,
            WireElement::QueryChannelRangeTLVs => SerializableType::TLVStream,
            WireElement::FullInformation => SerializableType::Byte,
            WireElement::SyncComplete => SerializableType::Byte,
            WireElement::EncodedShortIds => SerializableType::U16SizedBytes,
            WireElement::ReplyChannelRangeTLVs => SerializableType::TLVStream,
        }
    }
}

#[derive(Debug)]
pub struct WireFormatMessage {
    message_type: MessageTypeEnum,
    elements: HashMap<WireElement, SerializedContainer>,
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
                SerializableType::MessageType => {
                    let (obj, bytes) = MessageTypeElement::from_bytes(bytes).unwrap();
                    (SerializedContainer::MessageType(obj), bytes)
                }
                SerializableType::U16Element => {
                    let (obj, bytes) = U16SerializedElement::from_bytes(bytes).unwrap();
                    (SerializedContainer::U16Element(obj), bytes)
                }
                SerializableType::U16SizedBytes => {
                    let (obj, bytes) = U16SizedBytesElement::from_bytes(bytes).unwrap();
                    (SerializedContainer::U16SizedBytes(obj), bytes)
                }
                SerializableType::TLVStream => {
                    let (obj, bytes) = TLVStreamElement::from_bytes(bytes).unwrap();
                    (SerializedContainer::TLVStream(obj), bytes)
                }
                SerializableType::Signature => {
                    let (obj, bytes) = SignatureElement::from_bytes(bytes).unwrap();
                    (SerializedContainer::Signature(obj), bytes)
                }
                SerializableType::ChainHash => {
                    let (obj, bytes) = ChainHashElement::from_bytes(bytes).unwrap();
                    (SerializedContainer::ChainHash(obj), bytes)
                }
                SerializableType::ShortChannelID => {
                    let (obj, bytes) = ShortChannelIDElement::from_bytes(bytes).unwrap();
                    (SerializedContainer::ShortChannelID(obj), bytes)
                }
                SerializableType::Point => {
                    let (obj, bytes) = PointElement::from_bytes(bytes).unwrap();
                    (SerializedContainer::Point(obj), bytes)
                }
                SerializableType::RGBColor => {
                    let (obj, bytes) = RGBColorElement::from_bytes(bytes).unwrap();
                    (SerializedContainer::RGBColor(obj), bytes)
                }
                SerializableType::NodeAlias => {
                    let (obj, bytes) = NodeAliasElement::from_bytes(bytes).unwrap();
                    (SerializedContainer::NodeAlias(obj), bytes)
                }
                SerializableType::U32Element => {
                    let (obj, bytes) = U32SerializedElement::from_bytes(bytes).unwrap();
                    (SerializedContainer::U32Element(obj), bytes)
                }
                SerializableType::Byte => {
                    let (obj, bytes) = ByteElement::from_bytes(bytes).unwrap();
                    (SerializedContainer::Byte(obj), bytes)
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
