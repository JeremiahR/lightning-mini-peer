use crate::wire::{
    BytesSerializable, ChainHashElement, FeaturesStruct, GlobalFeaturesStruct, IgnoredStruct,
    LocalFeaturesStruct, MessageTypeWire, NumPongBytesStruct, PointElement, SerializationError,
    ShortChannelIDElement, SignatureElement, TLVStreamElement, TimestampElement,
    TimestampRangeElement,
};

use num_enum::TryFromPrimitive;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};
use tokio::time::timeout;

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

    pub fn from_int(n: u16) -> Option<Self> {
        MessageType::iter().find(|&variant| variant as u16 == n)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitMessage {
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

#[derive(Debug)]
pub struct PingMessage {
    pub num_pong_bytes: u16,
    pub ignored: Vec<u8>,
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

#[derive(Debug)]
pub struct PongMessage {
    ignored: Vec<u8>,
}

impl PongMessage {
    pub fn from_ping(ping: PingMessage) -> Self {
        PongMessage {
            ignored: vec![0; ping.num_pong_bytes as usize],
        }
    }
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

#[derive(Debug)]
pub struct ChannelAnnouncementMessage {
    node_signature_1: [u8; 64],
    node_signature_2: [u8; 64],
    bitcoin_signature_1: [u8; 64],
    bitcoin_signature_2: [u8; 64],
    features: Vec<u8>,
    chain_hash: [u8; 32],
    short_channel_id: [u8; 8],
    node_id_1: [u8; 33],
    node_id_2: [u8; 33],
    bitcoin_node_id_1: [u8; 33],
    bitcoin_node_id_2: [u8; 33],
}

impl BytesSerializable for ChannelAnnouncementMessage {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (_message, data) = MessageTypeWire::from_bytes(data)?;
        let (node_signature_1, data) = SignatureElement::from_bytes(data)?;
        let (node_signature_2, data) = SignatureElement::from_bytes(data)?;
        let (bitcoin_signature_1, data) = SignatureElement::from_bytes(data)?;
        let (bitcoin_signature_2, data) = SignatureElement::from_bytes(data)?;
        let (features, data) = FeaturesStruct::from_bytes(data)?;
        let (chain_hash, data) = ChainHashElement::from_bytes(data)?;
        let (short_channel_id, data) = ShortChannelIDElement::from_bytes(data)?;
        let (node_id_1, data) = PointElement::from_bytes(data)?;
        let (node_id_2, data) = PointElement::from_bytes(data)?;
        let (bitcoin_node_id_1, data) = PointElement::from_bytes(data)?;
        let (bitcoin_node_id_2, data) = PointElement::from_bytes(data)?;

        Ok((
            ChannelAnnouncementMessage {
                node_signature_1: node_signature_1.data,
                node_signature_2: node_signature_2.data,
                bitcoin_signature_1: bitcoin_signature_1.data,
                bitcoin_signature_2: bitcoin_signature_2.data,
                features: features.data,
                chain_hash: chain_hash.data,
                short_channel_id: short_channel_id.data,
                node_id_1: node_id_1.data,
                node_id_2: node_id_2.data,
                bitcoin_node_id_1: bitcoin_node_id_1.data,
                bitcoin_node_id_2: bitcoin_node_id_2.data,
            },
            data,
        ))
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(MessageTypeWire::new(MessageType::ChannelAnnouncement).to_bytes());
        bytes.extend(SignatureElement::new(self.node_signature_1).to_bytes());
        bytes.extend(SignatureElement::new(self.node_signature_2).to_bytes());
        bytes.extend(SignatureElement::new(self.bitcoin_signature_1).to_bytes());
        bytes.extend(SignatureElement::new(self.bitcoin_signature_2).to_bytes());
        bytes.extend(FeaturesStruct::new(self.features.clone()).to_bytes());
        bytes.extend(ChainHashElement::new(self.chain_hash).to_bytes());
        bytes.extend(ShortChannelIDElement::new(self.short_channel_id).to_bytes());
        bytes.extend(PointElement::new(self.node_id_1).to_bytes());
        bytes.extend(PointElement::new(self.node_id_2).to_bytes());
        bytes.extend(PointElement::new(self.bitcoin_node_id_1).to_bytes());
        bytes.extend(PointElement::new(self.bitcoin_node_id_2).to_bytes());
        bytes
    }
}

#[derive(Debug)]
pub struct GossipTimestampFilterMessage {
    chain_hash: [u8; 32],
    first_timestamp: u32,
    timestamp_range: u32,
}

impl BytesSerializable for GossipTimestampFilterMessage {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (_, data) = MessageTypeWire::from_bytes(data)?;
        let (chain_hash, data) = ChainHashElement::from_bytes(data)?;
        let (first_timestamp, data) = TimestampElement::from_bytes(data)?;
        let (timestamp_range, data) = TimestampRangeElement::from_bytes(data)?;

        Ok((
            GossipTimestampFilterMessage {
                chain_hash: chain_hash.data,
                first_timestamp: first_timestamp.value,
                timestamp_range: timestamp_range.value,
            },
            data,
        ))
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(MessageTypeWire::new(MessageType::GossipTimestampFilter).to_bytes());
        bytes.extend(
            ChainHashElement {
                data: self.chain_hash,
            }
            .to_bytes(),
        );
        bytes.extend(
            TimestampElement {
                value: self.first_timestamp,
            }
            .to_bytes(),
        );
        bytes.extend(
            TimestampRangeElement {
                value: self.timestamp_range,
            }
            .to_bytes(),
        );
        bytes
    }
}

#[derive(Debug)]
pub struct UnknownMessage {
    type_id: u16,
    data: Vec<u8>,
}

impl BytesSerializable for UnknownMessage {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (message, data) = MessageTypeWire::from_bytes(data)?;

        Ok((
            UnknownMessage {
                type_id: message.id,
                data: data.to_vec(),
            },
            &[],
        ))
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(MessageTypeWire { id: self.type_id }.to_bytes());
        bytes.extend(self.data.clone());
        bytes
    }
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
