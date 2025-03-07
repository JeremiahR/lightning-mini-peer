use crate::{
    node::Node,
    serialization::{
        Bytes3Element, BytesSerializable, ChainHashElement, FeaturesStruct, GlobalFeaturesStruct,
        IgnoredStruct, LocalFeaturesStruct, MessageTypeWire, NodeAddressesWire, NumPongBytesStruct,
        PointElementWire, SerializationError, ShortChannelIDElement, SignatureElement,
        SingleByteWire, TLVStreamElement, TimestampElement, TimestampRangeElement, U16IntWire,
        U16SizedBytesWire, U32IntWire, U64IntWire, Wire32Bytes,
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

impl BytesSerializable for InitMessage {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (_message, data) = MessageTypeWire::from_bytes(data)?;
        let (global_features, data) = GlobalFeaturesStruct::from_bytes(data)?;
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
                ignored: ignored.value,
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
                ignored: ignored.value,
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
    short_channel_id: ShortChannelIDElement,
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
        let (node_id_1, data) = PointElementWire::from_bytes(data)?;
        let (node_id_2, data) = PointElementWire::from_bytes(data)?;
        let (bitcoin_node_id_1, data) = PointElementWire::from_bytes(data)?;
        let (bitcoin_node_id_2, data) = PointElementWire::from_bytes(data)?;

        Ok((
            ChannelAnnouncementMessage {
                node_signature_1: node_signature_1.value,
                node_signature_2: node_signature_2.value,
                bitcoin_signature_1: bitcoin_signature_1.value,
                bitcoin_signature_2: bitcoin_signature_2.value,
                features: features.value,
                chain_hash: chain_hash.value,
                short_channel_id,
                node_id_1: node_id_1.value,
                node_id_2: node_id_2.value,
                bitcoin_node_id_1: bitcoin_node_id_1.value,
                bitcoin_node_id_2: bitcoin_node_id_2.value,
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
        bytes.extend(self.short_channel_id.to_bytes());
        bytes.extend(PointElementWire::new(self.node_id_1).to_bytes());
        bytes.extend(PointElementWire::new(self.node_id_2).to_bytes());
        bytes.extend(PointElementWire::new(self.bitcoin_node_id_1).to_bytes());
        bytes.extend(PointElementWire::new(self.bitcoin_node_id_2).to_bytes());
        bytes
    }
}

#[derive(Debug, Clone)]
pub struct GossipTimestampFilterMessage {
    pub chain_hash: [u8; 32],
    pub first_timestamp: u32,
    pub timestamp_range: u32,
}

impl BytesSerializable for GossipTimestampFilterMessage {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (_, data) = MessageTypeWire::from_bytes(data)?;
        let (chain_hash, data) = ChainHashElement::from_bytes(data)?;
        let (first_timestamp, data) = TimestampElement::from_bytes(data)?;
        let (timestamp_range, data) = TimestampRangeElement::from_bytes(data)?;

        Ok((
            GossipTimestampFilterMessage {
                chain_hash: chain_hash.value,
                first_timestamp: first_timestamp.value,
                timestamp_range: timestamp_range.value,
            },
            data,
        ))
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(MessageTypeWire::new(MessageType::GossipTimestampFilter).to_bytes());
        bytes.extend(ChainHashElement::new(self.chain_hash).to_bytes());
        bytes.extend(TimestampElement::new(self.first_timestamp).to_bytes());
        bytes.extend(TimestampRangeElement::new(self.timestamp_range).to_bytes());
        bytes
    }
}

#[derive(Debug)]
pub struct QueryChannelRangeMessage {
    chain_hash: [u8; 32],
    first_blocknum: u32,
    number_of_blocks: u32,
    query_range_tlvs: Vec<u8>,
}

impl BytesSerializable for QueryChannelRangeMessage {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (_, data) = MessageTypeWire::from_bytes(data)?;
        let (chain_hash, data) = ChainHashElement::from_bytes(data)?;
        let (first_blocknum, data) = U32IntWire::from_bytes(data)?;
        let (number_of_blocks, data) = U32IntWire::from_bytes(data)?;
        let (query_range_tlvs, data) = TLVStreamElement::from_bytes(data)?;

        Ok((
            QueryChannelRangeMessage {
                chain_hash: chain_hash.value,
                first_blocknum: first_blocknum.value,
                number_of_blocks: number_of_blocks.value,
                query_range_tlvs: query_range_tlvs.value,
            },
            data,
        ))
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(MessageTypeWire::new(MessageType::QueryChannelRange).to_bytes());
        bytes.extend(ChainHashElement::new(self.chain_hash).to_bytes());
        bytes.extend(U32IntWire::new(self.first_blocknum).to_bytes());
        bytes.extend(U32IntWire::new(self.number_of_blocks).to_bytes());
        bytes.extend(TLVStreamElement::new(self.query_range_tlvs.clone()).to_bytes());
        bytes
    }
}

#[derive(Debug)]
pub struct ReplyChannelRangeMessage {
    chain_hash: [u8; 32],
    first_blocknum: u32,
    number_of_blocks: u32,
    sync_complete: u8,
    encoded_short_ids: Vec<u8>,
    reply_channel_range_tlvs: Vec<u8>,
}

impl BytesSerializable for ReplyChannelRangeMessage {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (_, data) = MessageTypeWire::from_bytes(data)?;
        let (chain_hash, data) = ChainHashElement::from_bytes(data)?;
        let (first_blocknum, data) = U32IntWire::from_bytes(data)?;
        let (number_of_blocks, data) = U32IntWire::from_bytes(data)?;
        let (sync_complete, data) = SingleByteWire::from_bytes(data)?;
        let (encoded_short_ids, data) = U16SizedBytesWire::from_bytes(data)?;
        let (reply_channel_range_tlvs, data) = TLVStreamElement::from_bytes(data)?;

        Ok((
            ReplyChannelRangeMessage {
                chain_hash: chain_hash.value,
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
        bytes.extend(MessageTypeWire::new(MessageType::ReplyChannelRange).to_bytes());
        bytes.extend(ChainHashElement::new(self.chain_hash).to_bytes());
        bytes.extend(U32IntWire::new(self.first_blocknum).to_bytes());
        bytes.extend(U32IntWire::new(self.number_of_blocks).to_bytes());
        bytes.extend(SingleByteWire::new(self.sync_complete).to_bytes());
        bytes.extend(U16SizedBytesWire::new(self.encoded_short_ids.clone()).to_bytes());
        bytes.extend(TLVStreamElement::new(self.reply_channel_range_tlvs.clone()).to_bytes());
        bytes
    }
}

#[derive(Debug)]
pub struct NodeAnnouncementMessage {
    signature: [u8; 64],
    features: Vec<u8>,
    timestamp: u32,
    pub node_id: [u8; 33],
    rgb_color: [u8; 3],
    alias: [u8; 32],
    addresses: NodeAddressesWire,
}

impl NodeAnnouncementMessage {
    pub fn as_node(&self) -> Node {
        let ipv4addr = self.addresses.ipv4_addresses.first().unwrap();
        let ip_address = format!(
            "{}.{}.{}.{}",
            ipv4addr[0], ipv4addr[1], ipv4addr[2], ipv4addr[3]
        );
        let port = u16::from_be_bytes([ipv4addr[4], ipv4addr[5]]);
        Node {
            public_key: self.node_id,
            ip_address,
            port,
        }
    }
}

impl BytesSerializable for NodeAnnouncementMessage {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (_, data) = MessageTypeWire::from_bytes(data)?;
        let (signature, data) = SignatureElement::from_bytes(data)?;
        let (features, data) = U16SizedBytesWire::from_bytes(data)?;
        let (timestamp, data) = U32IntWire::from_bytes(data)?;
        let (node_id, data) = PointElementWire::from_bytes(data)?;
        let (rgb_color, data) = Bytes3Element::from_bytes(data)?;
        let (alias, data) = Wire32Bytes::from_bytes(data)?;
        let (addresses, data) = NodeAddressesWire::from_bytes(data)?;

        Ok((
            NodeAnnouncementMessage {
                signature: signature.value,
                features: features.value,
                timestamp: timestamp.value,
                node_id: node_id.value,
                rgb_color: rgb_color.value,
                alias: alias.value,
                addresses,
            },
            data,
        ))
    }

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(MessageTypeWire::new(MessageType::NodeAnnouncement).to_bytes());
        bytes.extend(SignatureElement::new(self.signature).to_bytes());
        bytes.extend(U16SizedBytesWire::new(self.features.clone()).to_bytes());
        bytes.extend(U32IntWire::new(self.timestamp).to_bytes());
        bytes.extend(PointElementWire::new(self.node_id).to_bytes());
        bytes.extend(Bytes3Element::new(self.rgb_color).to_bytes());
        bytes.extend(Wire32Bytes::new(self.alias).to_bytes());
        bytes.extend(self.addresses.to_bytes());
        bytes
    }
}

#[derive(Debug)]
pub struct ChannelUpdateMessage {
    signature: [u8; 64],
    chain_hash: [u8; 32],
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

impl BytesSerializable for ChannelUpdateMessage {
    fn from_bytes(data: &[u8]) -> Result<(Self, &[u8]), SerializationError> {
        let (_, data) = MessageTypeWire::from_bytes(data)?;
        let (signature, data) = SignatureElement::from_bytes(data).unwrap();
        let (chain_hash, data) = ChainHashElement::from_bytes(data).unwrap();
        let (short_channel_id, data) = ShortChannelIDElement::from_bytes(data).unwrap();
        let (timestamp, data) = TimestampElement::from_bytes(data).unwrap();
        let (message_flags, data) = SingleByteWire::from_bytes(data).unwrap();
        let (channel_flags, data) = SingleByteWire::from_bytes(data).unwrap();
        let (cltv_expiry_delta, data) = U16IntWire::from_bytes(data).unwrap();
        let (htlc_minimum_msat, data) = U64IntWire::from_bytes(data).unwrap();
        let (fee_base_msat, data) = U32IntWire::from_bytes(data).unwrap();
        let (fee_proportional_millionths, data) = U32IntWire::from_bytes(data).unwrap();
        let (htlc_maximum_msat, data) = U64IntWire::from_bytes(data).unwrap();

        Ok((
            ChannelUpdateMessage {
                signature: signature.value,
                chain_hash: chain_hash.value,
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
        bytes.extend(MessageTypeWire::new(MessageType::ChannelUpdate).to_bytes());
        bytes.extend(SignatureElement::new(self.signature).to_bytes());
        bytes.extend(ChainHashElement::new(self.chain_hash).to_bytes());
        bytes.extend(self.short_channel_id.to_bytes());
        bytes.extend(TimestampElement::new(self.timestamp).to_bytes());
        bytes.extend(SingleByteWire::new(self.message_flags).to_bytes());
        bytes.extend(SingleByteWire::new(self.channel_flags).to_bytes());
        bytes.extend(U16IntWire::new(self.cltv_expiry_delta).to_bytes());
        bytes.extend(U64IntWire::new(self.htlc_minimum_msat).to_bytes());
        bytes.extend(U32IntWire::new(self.fee_base_msat).to_bytes());
        bytes.extend(U32IntWire::new(self.fee_proportional_millionths).to_bytes());
        bytes.extend(U64IntWire::new(self.htlc_maximum_msat).to_bytes());
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
