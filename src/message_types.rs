// use strum::{EnumIter, IntoStaticStr};
use strum_macros::{EnumIter, IntoStaticStr};

#[derive(Debug, EnumIter, Clone, IntoStaticStr)]
pub enum MessageTypes {
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
