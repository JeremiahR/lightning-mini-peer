use std::collections::HashMap;

use crate::message_types::MessageType;
use crate::serialization::{
    ByteElement, ChainHashElement, MessageTypeStruct, NodeAliasElement, PointElement,
    RGBColorElement, Serializable, SerializedKind, SerializedTypeContainer, ShortChannelIDElement,
    SignatureElement, TLVStreamElement, U16SerializedElement, U16SizedBytesStruct,
    U32SerializedElement,
};

#[derive(Debug)]
pub enum MessageDecodeError {
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum LightningType {
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

impl LightningType {
    pub fn serialized_kind(element: LightningType) -> SerializedKind {
        match element {
            LightningType::MessageType => SerializedKind::MessageType,
            LightningType::GlobalFeatures => SerializedKind::U16SizedBytes,
            LightningType::LocalFeatures => SerializedKind::U16SizedBytes,
            LightningType::TLVStream => SerializedKind::TLVStream,
            LightningType::NumPongBytes => SerializedKind::U16Element,
            LightningType::Ignored => SerializedKind::U16SizedBytes,
            LightningType::Signature => SerializedKind::Signature,
            LightningType::NodeSignature1 => SerializedKind::Signature,
            LightningType::NodeSignature2 => SerializedKind::Signature,
            LightningType::BitcoinSignature1 => SerializedKind::Signature,
            LightningType::BitcoinSignature2 => SerializedKind::Signature,
            LightningType::Features => SerializedKind::U16SizedBytes,
            LightningType::ChainHash => SerializedKind::ChainHash,
            LightningType::ShortChannelID => SerializedKind::ShortChannelID,
            LightningType::NodeId => SerializedKind::Point,
            LightningType::NodeId1 => SerializedKind::Point,
            LightningType::NodeId2 => SerializedKind::Point,
            LightningType::BitcoinKey1 => SerializedKind::Point,
            LightningType::BitcoinKey2 => SerializedKind::Point,
            LightningType::Timestamp => SerializedKind::U32Element,
            LightningType::FirstTimestamp => SerializedKind::U32Element,
            LightningType::TimestampRange => SerializedKind::U32Element,
            LightningType::FirstBlockNum => SerializedKind::U32Element,
            LightningType::NumberOfBlocks => SerializedKind::U32Element,
            LightningType::RGBColor => SerializedKind::RGBColor,
            LightningType::NodeAlias => SerializedKind::NodeAlias,
            LightningType::Addresses => SerializedKind::U16SizedBytes,
            LightningType::QuerySortChannelIDsTLVS => SerializedKind::TLVStream,
            LightningType::QueryChannelRangeTLVs => SerializedKind::TLVStream,
            LightningType::FullInformation => SerializedKind::Byte,
            LightningType::SyncComplete => SerializedKind::Byte,
            LightningType::EncodedShortIds => SerializedKind::U16SizedBytes,
            LightningType::ReplyChannelRangeTLVs => SerializedKind::TLVStream,
        }
    }
}

#[derive(Debug)]
pub struct WireFormatMessage {
    message_type: MessageType,
    elements: HashMap<LightningType, SerializedTypeContainer>,
    element_order: Vec<LightningType>,
}

impl WireFormatMessage {
    pub fn get_structure(
        msg_type: u16,
    ) -> Result<(MessageType, Vec<LightningType>), MessageDecodeError> {
        let type_enum = MessageType::try_from(msg_type).unwrap();
        let wire_elements: Vec<LightningType> = match type_enum {
            MessageType::Init => vec![
                LightningType::MessageType,
                LightningType::GlobalFeatures,
                LightningType::LocalFeatures,
                LightningType::TLVStream,
            ],
            MessageType::Ping => vec![
                LightningType::MessageType,
                LightningType::NumPongBytes,
                LightningType::Ignored,
            ],
            MessageType::Pong => vec![LightningType::MessageType, LightningType::Ignored],
            MessageType::ChannelAnnouncement => vec![
                LightningType::NodeSignature1,
                LightningType::NodeSignature2,
                LightningType::BitcoinSignature1,
                LightningType::BitcoinSignature2,
                LightningType::Features,
                LightningType::ChainHash,
                LightningType::ShortChannelID,
                LightningType::NodeId1,
                LightningType::NodeId2,
                LightningType::BitcoinKey1,
                LightningType::BitcoinKey2,
            ],
            MessageType::NodeAnnouncement => vec![
                LightningType::Signature,
                LightningType::Features,
                LightningType::Timestamp,
                LightningType::NodeId,
                LightningType::RGBColor,
                LightningType::NodeAlias,
                LightningType::Addresses,
            ],
            MessageType::QueryShortChannelIds => vec![
                LightningType::ChainHash,
                LightningType::EncodedShortIds,
                LightningType::QuerySortChannelIDsTLVS,
            ],
            MessageType::ReplyShortChannelIdsEnd => {
                vec![LightningType::ChainHash, LightningType::FullInformation]
            }
            MessageType::QueryChannelRange => vec![
                LightningType::ChainHash,
                LightningType::FirstBlockNum,
                LightningType::NumberOfBlocks,
                LightningType::QueryChannelRangeTLVs,
            ],
            MessageType::ReplyChannelRange => vec![
                LightningType::ChainHash,
                LightningType::FirstBlockNum,
                LightningType::NumberOfBlocks,
                LightningType::SyncComplete,
                LightningType::EncodedShortIds,
                LightningType::ReplyChannelRangeTLVs,
            ],
            MessageType::GossipTimestampFilter => vec![
                LightningType::ChainHash,
                LightningType::FirstTimestamp,
                LightningType::TimestampRange,
            ],
            _ => vec![],
        };
        Ok((type_enum, wire_elements))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(WireFormatMessage, &[u8]), MessageDecodeError> {
        let (m, _) = MessageTypeStruct::from_bytes(bytes).unwrap();
        let (message_type, wire_elements) = WireFormatMessage::get_structure(m.id).unwrap();
        let mut elements = HashMap::new();
        let mut element_order = Vec::new();
        let mut bytes = bytes;
        for wire_element in wire_elements {
            let (obj, rem_bytes) = match LightningType::serialized_kind(wire_element.clone()) {
                SerializedKind::MessageType => {
                    let (obj, bytes) = MessageTypeStruct::from_bytes(bytes).unwrap();
                    (SerializedTypeContainer::MessageType(obj), bytes)
                }
                SerializedKind::U16Element => {
                    let (obj, bytes) = U16SerializedElement::from_bytes(bytes).unwrap();
                    (SerializedTypeContainer::U16Element(obj), bytes)
                }
                SerializedKind::U16SizedBytes => {
                    let (obj, bytes) = U16SizedBytesStruct::from_bytes(bytes).unwrap();
                    (SerializedTypeContainer::U16SizedBytes(obj), bytes)
                }
                SerializedKind::TLVStream => {
                    let (obj, bytes) = TLVStreamElement::from_bytes(bytes).unwrap();
                    (SerializedTypeContainer::TLVStream(obj), bytes)
                }
                SerializedKind::Signature => {
                    let (obj, bytes) = SignatureElement::from_bytes(bytes).unwrap();
                    (SerializedTypeContainer::Signature(obj), bytes)
                }
                SerializedKind::ChainHash => {
                    let (obj, bytes) = ChainHashElement::from_bytes(bytes).unwrap();
                    (SerializedTypeContainer::ChainHash(obj), bytes)
                }
                SerializedKind::ShortChannelID => {
                    let (obj, bytes) = ShortChannelIDElement::from_bytes(bytes).unwrap();
                    (SerializedTypeContainer::ShortChannelID(obj), bytes)
                }
                SerializedKind::Point => {
                    let (obj, bytes) = PointElement::from_bytes(bytes).unwrap();
                    (SerializedTypeContainer::Point(obj), bytes)
                }
                SerializedKind::RGBColor => {
                    let (obj, bytes) = RGBColorElement::from_bytes(bytes).unwrap();
                    (SerializedTypeContainer::RGBColor(obj), bytes)
                }
                SerializedKind::NodeAlias => {
                    let (obj, bytes) = NodeAliasElement::from_bytes(bytes).unwrap();
                    (SerializedTypeContainer::NodeAlias(obj), bytes)
                }
                SerializedKind::U32Element => {
                    let (obj, bytes) = U32SerializedElement::from_bytes(bytes).unwrap();
                    (SerializedTypeContainer::U32Element(obj), bytes)
                }
                SerializedKind::Byte => {
                    let (obj, bytes) = ByteElement::from_bytes(bytes).unwrap();
                    (SerializedTypeContainer::Byte(obj), bytes)
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
        assert_eq!(msg.message_type, MessageType::Init);
        // check that "type" is contained in msg.elements
        assert!(msg.elements.contains_key(&LightningType::MessageType));
        assert!(msg.elements.contains_key(&LightningType::GlobalFeatures));
        assert!(msg.elements.contains_key(&LightningType::LocalFeatures));
        // check serialization
        assert_eq!([msg.to_bytes(), remainder.to_vec()].concat(), initial_bytes);
    }
}
