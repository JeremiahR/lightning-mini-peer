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
enum MessageElement {
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

type MessageStructurePair = (MessageElement, SerializableTypes);

// And a list (Vec) of such tuples.
type StructurePairList = Vec<MessageStructurePair>;

#[derive(Debug)]
pub struct Message {
    message_type: MessageTypeEnum,
    elements: HashMap<MessageElement, SerializableElement>,
    element_order: Vec<MessageElement>,
}

impl Message {
    pub fn get_structure(
        msg_type: u16,
    ) -> Result<(MessageTypeEnum, StructurePairList), MessageDecodeError> {
        let type_enum = MessageTypeEnum::try_from(msg_type).unwrap();
        let structure_pairs = match type_enum {
            MessageTypeEnum::Init => vec![
                (MessageElement::MessageType, SerializableTypes::MessageType),
                (
                    MessageElement::GlobalFeatures,
                    SerializableTypes::U16SizedBytes,
                ),
                (
                    MessageElement::LocalFeatures,
                    SerializableTypes::U16SizedBytes,
                ),
                (MessageElement::TLVStream, SerializableTypes::TLVStream),
            ],
            MessageTypeEnum::Ping => vec![
                (MessageElement::MessageType, SerializableTypes::MessageType),
                (MessageElement::NumPongBytes, SerializableTypes::U16Element),
                (MessageElement::Ignored, SerializableTypes::U16SizedBytes),
            ],
            MessageTypeEnum::Pong => vec![
                (MessageElement::MessageType, SerializableTypes::MessageType),
                (MessageElement::Ignored, SerializableTypes::U16SizedBytes),
            ],
            MessageTypeEnum::ChannelAnnouncement => vec![
                (MessageElement::NodeSignature1, SerializableTypes::Signature),
                (MessageElement::NodeSignature2, SerializableTypes::Signature),
                (
                    MessageElement::BitcoinSignature1,
                    SerializableTypes::Signature,
                ),
                (
                    MessageElement::BitcoinSignature2,
                    SerializableTypes::Signature,
                ),
                (MessageElement::Features, SerializableTypes::U16SizedBytes),
                (MessageElement::ChainHash, SerializableTypes::ChainHash),
                (
                    MessageElement::ShortChannelID,
                    SerializableTypes::ShortChannelID,
                ),
                (MessageElement::NodeId1, SerializableTypes::Point),
                (MessageElement::NodeId2, SerializableTypes::Point),
                (MessageElement::BitcoinKey1, SerializableTypes::Point),
                (MessageElement::BitcoinKey2, SerializableTypes::Point),
            ],
            MessageTypeEnum::NodeAnnouncement => vec![
                (MessageElement::Signature, SerializableTypes::Signature),
                (MessageElement::Features, SerializableTypes::U16SizedBytes),
                (MessageElement::Timestamp, SerializableTypes::U32Element),
                (MessageElement::NodeId, SerializableTypes::Point),
                (MessageElement::RGBColor, SerializableTypes::RGBColor),
                (MessageElement::NodeAlias, SerializableTypes::NodeAlias),
                (MessageElement::Addresses, SerializableTypes::U16SizedBytes),
            ],
            MessageTypeEnum::QueryShortChannelIds => vec![
                (MessageElement::ChainHash, SerializableTypes::ChainHash),
                (
                    MessageElement::EncodedShortIds,
                    SerializableTypes::U16SizedBytes,
                ),
                (
                    MessageElement::QuerySortChannelIDsTLVS,
                    SerializableTypes::TLVStream,
                ),
            ],
            MessageTypeEnum::ReplyShortChannelIdsEnd => vec![
                (MessageElement::ChainHash, SerializableTypes::ChainHash),
                (MessageElement::FullInformation, SerializableTypes::Byte),
            ],
            MessageTypeEnum::QueryChannelRange => vec![
                (MessageElement::ChainHash, SerializableTypes::ChainHash),
                (MessageElement::FirstBlockNum, SerializableTypes::U32Element),
                (
                    MessageElement::NumberOfBlocks,
                    SerializableTypes::U32Element,
                ),
                (
                    MessageElement::QueryChannelRangeTLVs,
                    SerializableTypes::TLVStream,
                ),
            ],
            MessageTypeEnum::ReplyChannelRange => vec![
                (MessageElement::ChainHash, SerializableTypes::ChainHash),
                (MessageElement::FirstBlockNum, SerializableTypes::U32Element),
                (
                    MessageElement::NumberOfBlocks,
                    SerializableTypes::U32Element,
                ),
                (MessageElement::SyncComplete, SerializableTypes::Byte),
                (
                    MessageElement::EncodedShortIds,
                    SerializableTypes::U16Element,
                ),
                (
                    MessageElement::ReplyChannelRangeTLVs,
                    SerializableTypes::TLVStream,
                ),
            ],
            MessageTypeEnum::GossipTimestampFilter => vec![
                (MessageElement::ChainHash, SerializableTypes::ChainHash),
                (
                    MessageElement::FirstTimestamp,
                    SerializableTypes::U32Element,
                ),
                (
                    MessageElement::TimestampRange,
                    SerializableTypes::U32Element,
                ),
            ],
            _ => vec![],
        };
        Ok((type_enum, structure_pairs))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Message, &[u8]), MessageDecodeError> {
        let (m, _) = MessageTypeElement::from_bytes(bytes).unwrap();
        let (message_type, structure) = Message::get_structure(m.id).unwrap();
        let mut elements = HashMap::new();
        let mut element_order = Vec::new();
        let mut bytes = bytes;
        for (key, enum_type) in &structure {
            let (obj, rem_bytes) = match enum_type {
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
            elements.insert(key.clone(), obj);
            element_order.push(key.clone());
        }
        Ok((
            Message {
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
        let (msg, remainder) = Message::from_bytes(&initial_bytes).unwrap();
        assert_eq!(msg.message_type, MessageTypeEnum::Init);
        // check that "type" is contained in msg.elements
        assert!(msg.elements.contains_key(&MessageElement::MessageType));
        assert!(msg.elements.contains_key(&MessageElement::GlobalFeatures));
        assert!(msg.elements.contains_key(&MessageElement::LocalFeatures));
        // check serialization
        assert_eq!([msg.to_bytes(), remainder.to_vec()].concat(), initial_bytes);
    }
}
