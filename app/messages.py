from dataclasses import dataclass
from enum import Enum
from typing import Dict, List, Self, Tuple, Type, TypeAlias, cast

from app.message_elements import (
    ChainHashElement,
    EncodedShortChannelIdsElement,
    GlobalFeaturesElement,
    LocalFeaturesElement,
    MessageTypeElement,
    PointElement,
    RemainderElement,
    SerializedElement,
    ShortChannelIDElement,
    SignatureElement,
    SingleByteElement,
    U16Element,
    U16VarBytesElement,
    U32Element,
    U64Element,
)


class MessageProperty(Enum):
    """
    An ElementKey represents the unique names for datatypes which are encoded in documentation, for example see [channel_announcement](https://github.com/lightning/bolts/blob/master/07-routing-gossip.md#the-channel_announcement-message) in Bolt 7.
    """

    TYPE = "type"
    GLOBAL_FEATURES = "global_features"
    LOCAL_FEATURES = "local_features"
    NUM_PONG_BYTES = "num_pong_bytes"
    PING_OR_PONG_BYTES = "ping_or_pong_bytes"
    SIGNATURE = "signature"
    NODE_SIGNATURE_1 = "node_signature_1"
    NODE_SIGNATURE_2 = "node_signature_2"
    BITCOIN_SIGNATURE_1 = "bitcoin_signature_1"
    BITCOIN_SIGNATURE_2 = "bitcoin_signature_2"
    CHANNEL_FEATURES = "channel_features"
    CHAIN_HASH = "chain_hash"
    SHORT_CHANNEL_ID = "short_channel_id"
    NODE_ID_1 = "node_id_1"
    NODE_ID_2 = "node_id_2"
    BITCOIN_KEY_1 = "bitcoin_key_1"
    BITCOIN_KEY_2 = "bitcoin_key_2"
    FIRST_TIMESTAMP = "first_timestamp"
    TIMESTAMP_RANGE = "timestamp_range"
    ENCODED_SHORT_CHANNEL_IDS = "encoded_short_ids"
    FULL_INFORMATION = "full_information"
    TIMESTAMP = "timestamp"
    MESSAGE_FLAGS = "message_flags"
    CHANNEL_FLAGS = "channel_flags"
    CLTV_EXPIRY_DELTA = "cltv_expiry_delta"
    HTLC_MINIMUM_MSAT = "htlc_minimum_msat"
    FEE_BASE_MSAT = "fee_base_msat"
    FEE_PROPORTIONAL_MILLIONTHS = "fee_proportional_millionths"
    HTLC_MAXIMUM_MSAT = "htlc_maximum_msat"
    REMAINDER = "remainder"
    FIRST_BLOCK_NUM = "first_block_num"
    NUMBER_OF_BLOCKS = "number_of_blocks"
    SYNC_COMPLETE = "sync_complete"


KeyedElement: TypeAlias = Tuple[MessageProperty, Type[SerializedElement]]
MessagePropertiesDict: TypeAlias = Dict[MessageProperty, SerializedElement]


@dataclass
class Message:
    id: int
    name: str
    properties: MessagePropertiesDict

    @classmethod
    def features(cls) -> List[KeyedElement]:
        return [
            (MessageProperty.TYPE, MessageTypeElement),
        ]

    @classmethod
    def from_bytes(cls, data: bytes):
        properties = {}
        chunked = bytes(data)
        for key, feature in cls.features():
            el, chunked = feature.from_bytes(bytes(chunked))
            properties[key] = el
            if feature is MessageTypeElement:
                el = cast(MessageTypeElement, el)
                id: int = el.id
                name: str = el.name
        if len(chunked) > 0:
            properties[MessageProperty.REMAINDER], chunked = RemainderElement.from_bytes(chunked)
        assert len(chunked) == 0, f"Unexpected data left: {chunked}"
        return cls(id, name, properties)  # pyright: ignore

    def to_bytes(self) -> bytes:
        data = b""
        for key, feature in self.features():
            data += self.properties[key].to_bytes()
        if MessageProperty.REMAINDER in self.properties:
            data += self.properties[MessageProperty.REMAINDER].to_bytes()
        return data

    def __str__(self):
        out = []
        for key, property in self.properties.items():
            out.append(f"{key.value}: {property}")
        return f"{self.__class__.__name__}({', '.join(out)})"

    def __getattr__(self, name: str):
        """Fallback for dynamically created properties."""
        try:
            return self.properties[MessageProperty[name.upper()]]
        except KeyError:
            raise AttributeError(f"{name} not found")

    @property
    def type_id(self):
        return cast(MessageTypeElement, self.type).id

    @property
    def type_element(self):
        return cast(MessageTypeElement, self.type).name

    @property
    def type_name(self):
        return self.name

    @property
    def length(self):
        return len(self.to_bytes())


class InitMessage(Message):
    id = 16

    @classmethod
    def features(cls) -> List[KeyedElement]:
        return super().features() + [
            (MessageProperty.GLOBAL_FEATURES, U16VarBytesElement),
            (MessageProperty.LOCAL_FEATURES, U16VarBytesElement),
        ]

    @property
    def global_features(self):
        return cast(GlobalFeaturesElement, self.properties[MessageProperty.GLOBAL_FEATURES])

    @property
    def local_features(self):
        return cast(LocalFeaturesElement, self.properties[MessageProperty.LOCAL_FEATURES])


class PingMessage(Message):
    id: int = 18
    name: str = "ping"

    @classmethod
    def features(cls) -> List[KeyedElement]:
        return super().features() + [
            (MessageProperty.NUM_PONG_BYTES, U16Element),
            (MessageProperty.PING_OR_PONG_BYTES, U16VarBytesElement),
        ]

    @property
    def num_pong_bytes(self):
        return cast(U16Element, self.properties[MessageProperty.NUM_PONG_BYTES])

    @classmethod
    def create(cls, num_pong_bytes: int, message: bytes):
        return PingMessage(
            id=18,
            name="ping",
            properties={
                MessageProperty.TYPE: MessageTypeElement(id=18, name="ping"),
                MessageProperty.NUM_PONG_BYTES: U16Element(num_bytes=num_pong_bytes),
                MessageProperty.PING_OR_PONG_BYTES: U16VarBytesElement(len(message), data=message),
            },
        )


class PongMessage(Message):
    id = 19
    name = "pong"

    @classmethod
    def features(cls) -> List[KeyedElement]:
        return super().features() + [
            (MessageProperty.PING_OR_PONG_BYTES, U16VarBytesElement),
        ]

    @property
    def ping_or_pong_bytes(self):
        return cast(U16VarBytesElement, self.properties[MessageProperty.PING_OR_PONG_BYTES])

    @property
    def num_bytes(self):
        return self.ping_or_pong_bytes.num_bytes

    @classmethod
    def create_from_ping(cls, msg: PingMessage):
        return PongMessage(
            id=19,
            name="pong",
            properties={
                MessageProperty.TYPE: MessageTypeElement(id=19, name="pong"),
                MessageProperty.PING_OR_PONG_BYTES: U16VarBytesElement(
                    msg.num_pong_bytes.num_bytes,
                    data=b"0" * msg.num_pong_bytes.num_bytes,
                ),
            },
        )


class ChannelAnnouncementMessage(Message):
    id = 256
    name = "channel_announcement"

    @classmethod
    def features(cls) -> List[KeyedElement]:
        return super().features() + [
            (MessageProperty.NODE_SIGNATURE_1, SignatureElement),
            (MessageProperty.NODE_SIGNATURE_2, SignatureElement),
            (MessageProperty.BITCOIN_SIGNATURE_1, SignatureElement),
            (MessageProperty.BITCOIN_SIGNATURE_2, SignatureElement),
            (MessageProperty.CHANNEL_FEATURES, U16VarBytesElement),
            (MessageProperty.CHAIN_HASH, ChainHashElement),
            (MessageProperty.SHORT_CHANNEL_ID, ShortChannelIDElement),
            (MessageProperty.NODE_ID_1, PointElement),
            (MessageProperty.NODE_ID_2, PointElement),
            (MessageProperty.BITCOIN_KEY_1, PointElement),
            (MessageProperty.BITCOIN_KEY_2, PointElement),
        ]

    @property
    def node_signature_1(self):
        return cast(SignatureElement, self.properties[MessageProperty.NODE_SIGNATURE_1])

    @property
    def node_signature_2(self):
        return cast(SignatureElement, self.properties[MessageProperty.NODE_SIGNATURE_2])

    @property
    def bitcoin_signature_1(self):
        return cast(SignatureElement, self.properties[MessageProperty.BITCOIN_SIGNATURE_1])

    @property
    def bitcoin_signature_2(self):
        return cast(SignatureElement, self.properties[MessageProperty.BITCOIN_SIGNATURE_2])

    @property
    def channel_features(self):
        return cast(U16VarBytesElement, self.properties[MessageProperty.CHANNEL_FEATURES])

    @property
    def chain_hash(self):
        return cast(ChainHashElement, self.properties[MessageProperty.CHAIN_HASH])

    @property
    def short_channel_id(self):
        return cast(ShortChannelIDElement, self.properties[MessageProperty.SHORT_CHANNEL_ID])

    @property
    def node_id_1(self):
        return cast(PointElement, self.properties[MessageProperty.NODE_ID_1])

    @property
    def node_id_2(self):
        return cast(PointElement, self.properties[MessageProperty.NODE_ID_2])

    @property
    def bitcoin_key_1(self):
        return cast(PointElement, self.properties[MessageProperty.BITCOIN_KEY_1])

    @property
    def bitcoin_key_2(self):
        return cast(PointElement, self.properties[MessageProperty.BITCOIN_KEY_2])


class ChannelUpdateMessage(Message):
    id = 258
    name = "channel_update"

    @classmethod
    def features(cls) -> List[KeyedElement]:
        # https://github.com/lightning/bolts/blob/master/07-routing-gossip.md#the-channel_update-message
        return super().features() + [
            (MessageProperty.SIGNATURE, SignatureElement),
            (MessageProperty.CHAIN_HASH, ChainHashElement),
            (MessageProperty.SHORT_CHANNEL_ID, ShortChannelIDElement),
            (MessageProperty.TIMESTAMP, U32Element),
            (MessageProperty.MESSAGE_FLAGS, SingleByteElement),
            (MessageProperty.CHANNEL_FLAGS, SingleByteElement),
            (MessageProperty.CLTV_EXPIRY_DELTA, U16Element),
            (MessageProperty.HTLC_MINIMUM_MSAT, U64Element),
            (MessageProperty.FEE_BASE_MSAT, U32Element),
            (MessageProperty.FEE_PROPORTIONAL_MILLIONTHS, U32Element),
            (MessageProperty.HTLC_MAXIMUM_MSAT, U64Element),
        ]


class GossipTimestampFilterMessage(Message):
    id = 265
    name = "gossip_timestamp_filter"

    @classmethod
    def features(cls) -> List[KeyedElement]:
        return super().features() + [
            (MessageProperty.CHAIN_HASH, ChainHashElement),
            (MessageProperty.FIRST_TIMESTAMP, U32Element),
            (MessageProperty.TIMESTAMP_RANGE, U32Element),
        ]

    @property
    def chain_hash(self):
        return cast(ChainHashElement, self.properties[MessageProperty.CHAIN_HASH])

    @property
    def first_timestamp(self):
        return cast(U32Element, self.properties[MessageProperty.FIRST_TIMESTAMP])

    @property
    def timestamp_range(self):
        return cast(U32Element, self.properties[MessageProperty.TIMESTAMP_RANGE])


class GossipChannelIDQueryFlags(Enum):
    CHANNEL_ANNOUNCEMENTS = 0
    CHANNEL_UPDATES_NODE_1 = 1
    CHANNEL_UPDATES_NODE_2 = 2
    NODE_ANNOUNCEMENTS_NODE_1 = 3
    NODE_ANNOUNCEMENTS_NODE_2 = 4


class QueryShortChannelIDsMessage(Message):
    id = 261
    name = "query_short_channel_ids"

    @classmethod
    def features(cls) -> List[KeyedElement]:
        return super().features() + [
            (MessageProperty.CHAIN_HASH, ChainHashElement),
            (MessageProperty.ENCODED_SHORT_CHANNEL_IDS, EncodedShortChannelIdsElement),
        ]

    @property
    def chain_hash(self):
        return cast(ChainHashElement, self.properties[MessageProperty.CHAIN_HASH])

    @property
    def encoded_short_channel_ids(self):
        return cast(
            EncodedShortChannelIdsElement,
            self.properties[MessageProperty.ENCODED_SHORT_CHANNEL_IDS],
        )


class ReplyShortChannelIDsMessage(Message):
    id = 262
    name = "reply_short_channel_ids"

    @classmethod
    def features(cls) -> List[KeyedElement]:
        return super().features() + [
            (MessageProperty.CHAIN_HASH, ChainHashElement),
            (MessageProperty.FULL_INFORMATION, SingleByteElement),
        ]


class QueryChannelRangeMessage(Message):
    id = 263
    name = "query_channel_range"

    @classmethod
    def features(cls) -> List[KeyedElement]:
        return super().features() + [
            (MessageProperty.CHAIN_HASH, ChainHashElement),
            (MessageProperty.FIRST_BLOCK_NUM, U32Element),
            (MessageProperty.NUMBER_OF_BLOCKS, U32Element),
        ]

    @classmethod
    def create(cls, chain_hash: bytes, first_block_num: int, number_of_blocks: int) -> Self:
        return cls(
            263,
            "query_channel_range",
            {
                MessageProperty.TYPE: MessageTypeElement(263, "query_channel_range"),
                MessageProperty.CHAIN_HASH: ChainHashElement(chain_hash),
                MessageProperty.FIRST_BLOCK_NUM: U32Element(first_block_num),
                MessageProperty.NUMBER_OF_BLOCKS: U32Element(number_of_blocks),
            },
        )

    @property
    def chain_hash(self):
        return cast(ChainHashElement, self.properties[MessageProperty.CHAIN_HASH])

    @property
    def first_block_num(self):
        return cast(U32Element, self.properties[MessageProperty.FIRST_BLOCK_NUM])

    @property
    def number_of_blocks(self):
        return cast(U32Element, self.properties[MessageProperty.NUMBER_OF_BLOCKS])


class ReplyChannelRangeMessage(Message):
    id = 264
    name = "reply_channel_range"

    @classmethod
    def features(cls) -> List[KeyedElement]:
        return super().features() + [
            (MessageProperty.CHAIN_HASH, ChainHashElement),
            (MessageProperty.FIRST_BLOCK_NUM, U32Element),
            (MessageProperty.NUMBER_OF_BLOCKS, U32Element),
            (MessageProperty.SYNC_COMPLETE, SingleByteElement),
            (MessageProperty.ENCODED_SHORT_CHANNEL_IDS, EncodedShortChannelIdsElement),
        ]

    @property
    def chain_hash(self):
        return cast(ChainHashElement, self.properties[MessageProperty.CHAIN_HASH])

    @property
    def first_block_num(self):
        return cast(U32Element, self.properties[MessageProperty.FIRST_BLOCK_NUM])

    @property
    def number_of_blocks(self):
        return cast(U32Element, self.properties[MessageProperty.NUMBER_OF_BLOCKS])

    @property
    def sync_complete(self):
        return cast(SingleByteElement, self.properties[MessageProperty.SYNC_COMPLETE])

    @property
    def encoded_short_channel_ids(self):
        return cast(
            EncodedShortChannelIdsElement,
            self.properties[MessageProperty.ENCODED_SHORT_CHANNEL_IDS],
        )
