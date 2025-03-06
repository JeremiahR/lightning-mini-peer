use crate::wire::{
    BytesSerializable, GlobalFeaturesStruct, IgnoredStruct, LocalFeaturesStruct, MessageTypeWire,
    NumPongBytesStruct, SerializationError, TLVStreamElement,
};

use num_enum::TryFromPrimitive;
use strum_macros::{EnumIter, IntoStaticStr};

#[derive(Debug, EnumIter, Copy, Clone, Eq, PartialEq, Hash, IntoStaticStr, TryFromPrimitive)]
#[repr(u16)]
pub enum MessageType {
    Unknown = 0,
    Warning = 1,
    Stfu = 2,
    // connection and keep alive
    Init = 16,
    Error = 17,
    Ping = 18,
    Pong = 19,
    // channel
    OpenChannel = 32,
    AcceptChannel = 33,
    FundingCreated = 34,
    FundingSigned = 35,
    ChannelReady = 36,
    Shutdown = 38,
    ClosingSigned = 39,
    ClosingComplete = 40,
    ClosingSig = 41,
    OpenChannel2 = 64,
    AcceptChannel2 = 65,
    // tx updates
    TxAddInput = 66,
    TxAddOutput = 67,
    TxRemoveInput = 68,
    TxRemoveOutput = 69,
    TxComplete = 70,
    TxSignatures = 71,
    TxInitRbf = 72,
    TxAckRbf = 73,
    TxAbort = 74,
    // channel updates and htlc management
    UpdateAddHTLC = 128,
    UpdateFulfillHTLC = 130,
    UpdateFailHTLC = 131,
    CommitmentSigned = 132,
    RevokeAndAck = 133,
    UpdateFee = 134,
    UpdateFailMalformedHTLC = 135,
    ChannelReestablish = 136,
    // network announcements
    ChannelAnnouncement = 256,
    NodeAnnouncement = 257,
    ChannelUpdate = 258,
    AnnouncementSignatures = 259,
    // gossip
    QueryShortChannelIds = 261,
    ReplyShortChannelIdsEnd = 262,
    QueryChannelRange = 263,
    ReplyChannelRange = 264,
    GossipTimestampFilter = 265,
}

impl MessageType {
    pub fn as_u16(&self) -> u16 {
        *self as u16
    }
}

// struct types
pub type Signature = [u8; 64];

#[derive(Debug, Clone, PartialEq, Eq)]
struct InitMessage {
    global_features: Vec<u8>,
    local_features: Vec<u8>,
    tlv: Vec<u8>,
}

impl BytesSerializable for InitMessage {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (_message, data) = MessageTypeWire::from_bytes(data)?;
        let (global_features, data) = GlobalFeaturesStruct::from_bytes(data)?;
        let (local_features, data) = LocalFeaturesStruct::from_bytes(data)?;
        let (tlv, data) = TLVStreamElement::from_bytes(data)?;
        Ok((
            InitMessage {
                global_features: global_features.data,
                local_features: local_features.data,
                tlv: tlv.data,
            },
            data,
        ))
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(MessageTypeWire::new(MessageType::Init).to_bytes());
        bytes.extend(GlobalFeaturesStruct::new(self.global_features.clone()).to_bytes());
        bytes.extend(LocalFeaturesStruct::new(self.local_features.clone()).to_bytes());
        bytes.extend(TLVStreamElement::new(self.tlv.clone()).to_bytes());
        bytes
    }
}

struct PingMessage {
    num_pong_bytes: u16,
    ignored: Vec<u8>,
}

impl BytesSerializable for PingMessage {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (_message, data) = MessageTypeWire::from_bytes(data)?;
        let (num_pong_bytes, data) = NumPongBytesStruct::from_bytes(data)?;
        let (ignored, data) = IgnoredStruct::from_bytes(data)?;
        Ok((
            PingMessage {
                num_pong_bytes: num_pong_bytes.value,
                ignored: ignored.data,
            },
            data,
        ))
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(MessageTypeWire::new(MessageType::Ping).to_bytes());
        bytes.extend(NumPongBytesStruct::new(self.num_pong_bytes).to_bytes());
        bytes.extend(IgnoredStruct::new(self.ignored.clone()).to_bytes());
        bytes
    }
}

struct PongMessage {
    ignored: Vec<u8>,
}

impl BytesSerializable for PongMessage {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (_message, data) = MessageTypeWire::from_bytes(data)?;
        let (ignored, data) = IgnoredStruct::from_bytes(data)?;
        Ok((
            PongMessage {
                ignored: ignored.data,
            },
            data,
        ))
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(MessageTypeWire::new(MessageType::Pong).to_bytes());
        bytes.extend(IgnoredStruct::new(self.ignored.clone()).to_bytes());
        bytes
    }
}

struct ChannelAnnouncementMessage {
    node_signature_1: [u8; 64],
    node_signature_2: [u8; 64],
    bitcoin_signature_1: [u8; 64],
    bitcoin_signature_2: [u8; 64],
}

#[test]
fn test_decode_init_message() {
    let initial_bytes = hex::decode("001000021100000708a0880a8a59a1012006226e46111a0b59caaf126043eb5bbf28c34f3a5e332a1fc7b2b73cf188910f2d7ef99482067a1b72fe9e411d37be8c").unwrap();
    let (msg, remainder) = InitMessage::from_bytes(&initial_bytes).unwrap();
    assert!(!msg.global_features.is_empty());
    assert!(!msg.local_features.is_empty());
    // check serialization
    assert_eq!([msg.to_bytes(), remainder.to_vec()].concat(), initial_bytes);
}
