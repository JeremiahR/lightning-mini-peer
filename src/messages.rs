use crate::{
    node::Node,
    serialization::{
        ChainHashElement, FeaturesElement, GlobalFeaturesElement, IgnoredBytesElement,
        LocalFeaturesStruct, MessageTypeElement, NodeAddressesElement, NodeAliasElement,
        NumPongBytesElement, PointElement, SerializableToBytes, SerializationError,
        ShortChannelIDElement, SignatureElement, TLVStreamElement, TimestampElement,
        TimestampRangeElement, Wire1Byte, Wire3Bytes, WireU16Int, WireU16SizedBytes, WireU32Int,
        WireU64Int,
    },
};

use num_enum::TryFromPrimitive;
use strum::IntoEnumIterator;
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

impl SerializableToBytes for InitMessage {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (_message, data) = MessageTypeElement::from_bytes(data)?;
        let (global_features, data) = GlobalFeaturesElement::from_bytes(data)?;
        let (local_features, data) = LocalFeaturesStruct::from_bytes(data)?;
        let (tlv, data) = TLVStreamElement::from_bytes(data)?;
        Ok((
            InitMessage {
                global_features: global_features.value,
                local_features: local_features.value,
                tlv: tlv.value,
            },
            data,
        ))
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(MessageTypeElement::new(MessageType::Init).to_bytes());
        bytes.extend(GlobalFeaturesElement::new(self.global_features.clone()).to_bytes());
        bytes.extend(LocalFeaturesStruct::new(self.local_features.clone()).to_bytes());
        bytes.extend(TLVStreamElement::new(self.tlv.clone()).to_bytes());
        bytes
    }
}

#[derive(Debug)]
pub struct PingMessage {
    pub num_pong_bytes: u16,
    pub ignored: IgnoredBytesElement,
}

impl SerializableToBytes for PingMessage {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (_message, data) = MessageTypeElement::from_bytes(data)?;
        let (num_pong_bytes, data) = NumPongBytesElement::from_bytes(data)?;
        let (ignored, data) = IgnoredBytesElement::from_bytes(data)?;
        Ok((
            PingMessage {
                num_pong_bytes: num_pong_bytes.value,
                ignored,
            },
            data,
        ))
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(MessageTypeElement::new(MessageType::Ping).to_bytes());
        bytes.extend(NumPongBytesElement::new(self.num_pong_bytes).to_bytes());
        bytes.extend(self.ignored.to_bytes());
        bytes
    }
}

#[derive(Debug)]
pub struct PongMessage {
    ignored: IgnoredBytesElement,
}

impl PongMessage {
    pub fn from_ping(ping: PingMessage) -> Self {
        PongMessage {
            ignored: IgnoredBytesElement::new(vec![0; ping.num_pong_bytes as usize]),
        }
    }
}

impl SerializableToBytes for PongMessage {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (_message, data) = MessageTypeElement::from_bytes(data)?;
        let (ignored, data) = IgnoredBytesElement::from_bytes(data)?;
        Ok((PongMessage { ignored }, data))
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(MessageTypeElement::new(MessageType::Pong).to_bytes());
        bytes.extend(self.ignored.to_bytes());
        bytes
    }
}

#[derive(Debug)]
pub struct ChannelAnnouncementMessage {
    node_signature_1: SignatureElement,
    node_signature_2: SignatureElement,
    bitcoin_signature_1: SignatureElement,
    bitcoin_signature_2: SignatureElement,
    features: Vec<u8>,
    chain_hash: ChainHashElement,
    short_channel_id: ShortChannelIDElement,
    node_id_1: PointElement,
    node_id_2: PointElement,
    bitcoin_node_id_1: PointElement,
    bitcoin_node_id_2: PointElement,
}

impl SerializableToBytes for ChannelAnnouncementMessage {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (_message, data) = MessageTypeElement::from_bytes(data)?;
        let (node_signature_1, data) = SignatureElement::from_bytes(data)?;
        let (node_signature_2, data) = SignatureElement::from_bytes(data)?;
        let (bitcoin_signature_1, data) = SignatureElement::from_bytes(data)?;
        let (bitcoin_signature_2, data) = SignatureElement::from_bytes(data)?;
        let (features, data) = FeaturesElement::from_bytes(data)?;
        let (chain_hash, data) = ChainHashElement::from_bytes(data)?;
        let (short_channel_id, data) = ShortChannelIDElement::from_bytes(data)?;
        let (node_id_1, data) = PointElement::from_bytes(data)?;
        let (node_id_2, data) = PointElement::from_bytes(data)?;
        let (bitcoin_node_id_1, data) = PointElement::from_bytes(data)?;
        let (bitcoin_node_id_2, data) = PointElement::from_bytes(data)?;

        Ok((
            ChannelAnnouncementMessage {
                node_signature_1,
                node_signature_2,
                bitcoin_signature_1,
                bitcoin_signature_2,
                features: features.value,
                chain_hash,
                short_channel_id,
                node_id_1,
                node_id_2,
                bitcoin_node_id_1,
                bitcoin_node_id_2,
            },
            data,
        ))
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(MessageTypeElement::new(MessageType::ChannelAnnouncement).to_bytes());
        bytes.extend(self.node_signature_1.to_bytes());
        bytes.extend(self.node_signature_2.to_bytes());
        bytes.extend(self.bitcoin_signature_1.to_bytes());
        bytes.extend(self.bitcoin_signature_2.to_bytes());
        bytes.extend(FeaturesElement::new(self.features.clone()).to_bytes());
        bytes.extend(self.chain_hash.to_bytes());
        bytes.extend(self.short_channel_id.to_bytes());
        bytes.extend(self.node_id_1.to_bytes());
        bytes.extend(self.node_id_2.to_bytes());
        bytes.extend(self.bitcoin_node_id_1.to_bytes());
        bytes.extend(self.bitcoin_node_id_2.to_bytes());
        bytes
    }
}

#[derive(Debug, Clone)]
pub struct GossipTimestampFilterMessage {
    pub chain_hash: ChainHashElement,
    pub first_timestamp: u32,
    pub timestamp_range: u32,
}

impl SerializableToBytes for GossipTimestampFilterMessage {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (_, data) = MessageTypeElement::from_bytes(data)?;
        let (chain_hash, data) = ChainHashElement::from_bytes(data)?;
        let (first_timestamp, data) = TimestampElement::from_bytes(data)?;
        let (timestamp_range, data) = TimestampRangeElement::from_bytes(data)?;

        Ok((
            GossipTimestampFilterMessage {
                chain_hash,
                first_timestamp: first_timestamp.value,
                timestamp_range: timestamp_range.value,
            },
            data,
        ))
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(MessageTypeElement::new(MessageType::GossipTimestampFilter).to_bytes());
        bytes.extend(self.chain_hash.to_bytes());
        bytes.extend(TimestampElement::new(self.first_timestamp).to_bytes());
        bytes.extend(TimestampRangeElement::new(self.timestamp_range).to_bytes());
        bytes
    }
}

#[derive(Debug)]
pub struct QueryChannelRangeMessage {
    chain_hash: ChainHashElement,
    first_blocknum: u32,
    number_of_blocks: u32,
    query_range_tlvs: Vec<u8>,
}

impl SerializableToBytes for QueryChannelRangeMessage {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (_, data) = MessageTypeElement::from_bytes(data)?;
        let (chain_hash, data) = ChainHashElement::from_bytes(data)?;
        let (first_blocknum, data) = WireU32Int::from_bytes(data)?;
        let (number_of_blocks, data) = WireU32Int::from_bytes(data)?;
        let (query_range_tlvs, data) = TLVStreamElement::from_bytes(data)?;

        Ok((
            QueryChannelRangeMessage {
                chain_hash,
                first_blocknum: first_blocknum.value,
                number_of_blocks: number_of_blocks.value,
                query_range_tlvs: query_range_tlvs.value,
            },
            data,
        ))
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(MessageTypeElement::new(MessageType::QueryChannelRange).to_bytes());
        bytes.extend(self.chain_hash.to_bytes());
        bytes.extend(WireU32Int::new(self.first_blocknum).to_bytes());
        bytes.extend(WireU32Int::new(self.number_of_blocks).to_bytes());
        bytes.extend(TLVStreamElement::new(self.query_range_tlvs.clone()).to_bytes());
        bytes
    }
}

#[derive(Debug)]
pub struct ReplyChannelRangeMessage {
    chain_hash: ChainHashElement,
    first_blocknum: u32,
    number_of_blocks: u32,
    sync_complete: u8,
    encoded_short_ids: Vec<u8>,
    reply_channel_range_tlvs: Vec<u8>,
}

impl SerializableToBytes for ReplyChannelRangeMessage {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (_, data) = MessageTypeElement::from_bytes(data)?;
        let (chain_hash, data) = ChainHashElement::from_bytes(data)?;
        let (first_blocknum, data) = WireU32Int::from_bytes(data)?;
        let (number_of_blocks, data) = WireU32Int::from_bytes(data)?;
        let (sync_complete, data) = Wire1Byte::from_bytes(data)?;
        let (encoded_short_ids, data) = WireU16SizedBytes::from_bytes(data)?;
        let (reply_channel_range_tlvs, data) = TLVStreamElement::from_bytes(data)?;

        Ok((
            ReplyChannelRangeMessage {
                chain_hash,
                first_blocknum: first_blocknum.value,
                number_of_blocks: number_of_blocks.value,
                sync_complete: sync_complete.value,
                encoded_short_ids: encoded_short_ids.value,
                reply_channel_range_tlvs: reply_channel_range_tlvs.value,
            },
            data,
        ))
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(MessageTypeElement::new(MessageType::ReplyChannelRange).to_bytes());
        bytes.extend(self.chain_hash.to_bytes());
        bytes.extend(WireU32Int::new(self.first_blocknum).to_bytes());
        bytes.extend(WireU32Int::new(self.number_of_blocks).to_bytes());
        bytes.extend(Wire1Byte::new(self.sync_complete).to_bytes());
        bytes.extend(WireU16SizedBytes::new(self.encoded_short_ids.clone()).to_bytes());
        bytes.extend(TLVStreamElement::new(self.reply_channel_range_tlvs.clone()).to_bytes());
        bytes
    }
}

#[derive(Debug)]
pub struct NodeAnnouncementMessage {
    signature: SignatureElement,
    features: Vec<u8>,
    timestamp: u32,
    pub node_id: PointElement,
    rgb_color: [u8; 3],
    alias: NodeAliasElement,
    addresses: NodeAddressesElement,
}

impl NodeAnnouncementMessage {
    pub fn as_node(&self) -> Option<Node> {
        let ipv4addr = match self.addresses.ipv4_addresses.first() {
            Some(ipv4addr) => ipv4addr,
            None => return None,
        };
        let ip_address = format!(
            "{}.{}.{}.{}",
            ipv4addr[0], ipv4addr[1], ipv4addr[2], ipv4addr[3]
        );
        let port = u16::from_be_bytes([ipv4addr[4], ipv4addr[5]]);
        Some(Node {
            public_key: self.node_id.value,
            ip_address,
            port,
        })
    }
}

impl SerializableToBytes for NodeAnnouncementMessage {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (_, data) = MessageTypeElement::from_bytes(data)?;
        let (signature, data) = SignatureElement::from_bytes(data)?;
        let (features, data) = WireU16SizedBytes::from_bytes(data)?;
        let (timestamp, data) = WireU32Int::from_bytes(data)?;
        let (node_id, data) = PointElement::from_bytes(data)?;
        let (rgb_color, data) = Wire3Bytes::from_bytes(data)?;
        let (alias, data) = NodeAliasElement::from_bytes(data)?;
        let (addresses, data) = NodeAddressesElement::from_bytes(data)?;

        Ok((
            NodeAnnouncementMessage {
                signature,
                features: features.value,
                timestamp: timestamp.value,
                node_id,
                rgb_color: rgb_color.value,
                alias,
                addresses,
            },
            data,
        ))
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(MessageTypeElement::new(MessageType::NodeAnnouncement).to_bytes());
        bytes.extend(self.signature.to_bytes());
        bytes.extend(WireU16SizedBytes::new(self.features.clone()).to_bytes());
        bytes.extend(WireU32Int::new(self.timestamp).to_bytes());
        bytes.extend(self.node_id.to_bytes());
        bytes.extend(Wire3Bytes::new(self.rgb_color).to_bytes());
        bytes.extend(self.alias.to_bytes());
        bytes.extend(self.addresses.to_bytes());
        bytes
    }
}

#[derive(Debug)]
pub struct ChannelUpdateMessage {
    signature: SignatureElement,
    chain_hash: ChainHashElement,
    short_channel_id: ShortChannelIDElement,
    timestamp: u32,
    message_flags: u8,
    channel_flags: u8,
    cltv_expiry_delta: u16,
    htlc_minimum_msat: u64,
    fee_base_msat: u32,
    fee_proportional_millionths: u32,
    htlc_maximum_msat: u64,
}

impl SerializableToBytes for ChannelUpdateMessage {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (_, data) = MessageTypeElement::from_bytes(data)?;
        let (signature, data) = SignatureElement::from_bytes(data).unwrap();
        let (chain_hash, data) = ChainHashElement::from_bytes(data).unwrap();
        let (short_channel_id, data) = ShortChannelIDElement::from_bytes(data).unwrap();
        let (timestamp, data) = TimestampElement::from_bytes(data).unwrap();
        let (message_flags, data) = Wire1Byte::from_bytes(data).unwrap();
        let (channel_flags, data) = Wire1Byte::from_bytes(data).unwrap();
        let (cltv_expiry_delta, data) = WireU16Int::from_bytes(data).unwrap();
        let (htlc_minimum_msat, data) = WireU64Int::from_bytes(data).unwrap();
        let (fee_base_msat, data) = WireU32Int::from_bytes(data).unwrap();
        let (fee_proportional_millionths, data) = WireU32Int::from_bytes(data).unwrap();
        let (htlc_maximum_msat, data) = WireU64Int::from_bytes(data).unwrap();

        Ok((
            ChannelUpdateMessage {
                signature,
                chain_hash,
                short_channel_id,
                timestamp: timestamp.value,
                message_flags: message_flags.value,
                channel_flags: channel_flags.value,
                cltv_expiry_delta: cltv_expiry_delta.value,
                htlc_minimum_msat: htlc_minimum_msat.value,
                fee_base_msat: fee_base_msat.value,
                fee_proportional_millionths: fee_proportional_millionths.value,
                htlc_maximum_msat: htlc_maximum_msat.value,
            },
            data,
        ))
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(MessageTypeElement::new(MessageType::ChannelUpdate).to_bytes());
        bytes.extend(self.signature.to_bytes());
        bytes.extend(self.chain_hash.to_bytes());
        bytes.extend(self.short_channel_id.to_bytes());
        bytes.extend(TimestampElement::new(self.timestamp).to_bytes());
        bytes.extend(Wire1Byte::new(self.message_flags).to_bytes());
        bytes.extend(Wire1Byte::new(self.channel_flags).to_bytes());
        bytes.extend(WireU16Int::new(self.cltv_expiry_delta).to_bytes());
        bytes.extend(WireU64Int::new(self.htlc_minimum_msat).to_bytes());
        bytes.extend(WireU32Int::new(self.fee_base_msat).to_bytes());
        bytes.extend(WireU32Int::new(self.fee_proportional_millionths).to_bytes());
        bytes.extend(WireU64Int::new(self.htlc_maximum_msat).to_bytes());
        bytes
    }
}

#[derive(Debug)]
pub struct UnknownMessage {
    type_id: u16,
    data: Vec<u8>,
}

impl SerializableToBytes for UnknownMessage {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (message, data) = MessageTypeElement::from_bytes(data)?;

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
        bytes.extend(MessageTypeElement { id: self.type_id }.to_bytes());
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
