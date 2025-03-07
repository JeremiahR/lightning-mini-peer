use crate::messages::{
    ChannelAnnouncementMessage, ChannelUpdateMessage, GossipTimestampFilterMessage, InitMessage,
    MessageType, NodeAnnouncementMessage, PingMessage, PongMessage, QueryChannelRangeMessage,
    ReplyChannelRangeMessage, UnknownMessage,
};
use crate::serialization::BytesSerializable;
use crate::serialization::MessageTypeWire;

#[derive(Debug)]
pub enum MessageDecoderError {
    Error,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum MessageContainer {
    Init(InitMessage),
    Ping(PingMessage),
    Pong(PongMessage),
    ChannelAnnouncement(ChannelAnnouncementMessage),
    NodeAnnouncement(NodeAnnouncementMessage),
    GossipTimestampFilter(GossipTimestampFilterMessage),
    QueryChannelRange(QueryChannelRangeMessage),
    ReplyChannelRange(ReplyChannelRangeMessage),
    ChannelUpdate(ChannelUpdateMessage),
    Unknown(UnknownMessage),
}

impl MessageContainer {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            MessageContainer::Init(message) => message.to_bytes(),
            MessageContainer::Ping(message) => message.to_bytes(),
            MessageContainer::Pong(message) => message.to_bytes(),
            MessageContainer::ChannelAnnouncement(message) => message.to_bytes(),
            MessageContainer::NodeAnnouncement(message) => message.to_bytes(),
            MessageContainer::GossipTimestampFilter(message) => message.to_bytes(),
            MessageContainer::QueryChannelRange(message) => message.to_bytes(),
            MessageContainer::ReplyChannelRange(message) => message.to_bytes(),
            MessageContainer::ChannelUpdate(message) => message.to_bytes(),
            MessageContainer::Unknown(message) => message.to_bytes(),
        }
    }
}

pub struct MessageDecoder {}

impl MessageDecoder {
    pub fn from_bytes(bytes: &[u8]) -> Result<(MessageContainer, &[u8]), MessageDecoderError> {
        let (message_type_struct, _) = match MessageTypeWire::from_bytes(bytes) {
            Ok(message_type) => message_type,
            Err(_) => return Err(MessageDecoderError::Error),
        };
        let message_type = MessageType::from_int(message_type_struct.id).unwrap();
        match message_type {
            MessageType::Init => {
                let (message, data) = match InitMessage::from_bytes(bytes) {
                    Ok(x) => x,
                    Err(_) => return Err(MessageDecoderError::Error),
                };
                Ok((MessageContainer::Init(message), data))
            }
            MessageType::Ping => {
                let (message, data) = match PingMessage::from_bytes(bytes) {
                    Ok(x) => x,
                    Err(_) => return Err(MessageDecoderError::Error),
                };
                Ok((MessageContainer::Ping(message), data))
            }
            MessageType::Pong => {
                let (message, data) = match PongMessage::from_bytes(bytes) {
                    Ok(x) => x,
                    Err(_) => return Err(MessageDecoderError::Error),
                };
                Ok((MessageContainer::Pong(message), data))
            }
            MessageType::ChannelAnnouncement => {
                let (message, data) = match ChannelAnnouncementMessage::from_bytes(bytes) {
                    Ok(x) => x,
                    Err(_) => return Err(MessageDecoderError::Error),
                };
                Ok((MessageContainer::ChannelAnnouncement(message), data))
            }
            MessageType::NodeAnnouncement => {
                let (message, data) = match NodeAnnouncementMessage::from_bytes(bytes) {
                    Ok(x) => x,
                    Err(_) => return Err(MessageDecoderError::Error),
                };
                Ok((MessageContainer::NodeAnnouncement(message), data))
            }
            MessageType::GossipTimestampFilter => {
                let (message, data) = match GossipTimestampFilterMessage::from_bytes(bytes) {
                    Ok(x) => x,
                    Err(_) => return Err(MessageDecoderError::Error),
                };
                Ok((MessageContainer::GossipTimestampFilter(message), data))
            }
            MessageType::ReplyChannelRange => {
                let (message, data) = match ReplyChannelRangeMessage::from_bytes(bytes) {
                    Ok(x) => x,
                    Err(_) => return Err(MessageDecoderError::Error),
                };
                Ok((MessageContainer::ReplyChannelRange(message), data))
            }
            MessageType::QueryChannelRange => {
                let (message, data) = match QueryChannelRangeMessage::from_bytes(bytes) {
                    Ok(x) => x,
                    Err(_) => return Err(MessageDecoderError::Error),
                };
                Ok((MessageContainer::QueryChannelRange(message), data))
            }
            MessageType::ChannelUpdate => {
                let (message, data) = match ChannelUpdateMessage::from_bytes(bytes) {
                    Ok(x) => x,
                    Err(_) => return Err(MessageDecoderError::Error),
                };
                Ok((MessageContainer::ChannelUpdate(message), data))
            }
            _ => {
                let (message, data) = match UnknownMessage::from_bytes(bytes) {
                    Ok(x) => x,
                    Err(_) => return Err(MessageDecoderError::Error),
                };
                Ok((MessageContainer::Unknown(message), data))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    use super::*;

    fn read_example_messages() -> Vec<String> {
        // open examples file
        let f = File::open("test/examples").unwrap();
        let reader = BufReader::new(f);
        let lines = reader.lines();
        // return a vec of strings
        lines.map(|line| line.unwrap()).collect()
    }

    #[test]
    fn test_messages_deserialize_and_serialize() {
        for line in read_example_messages() {
            let initial_bytes = hex::decode(line).unwrap();
            let (message_type_struct, _) =
                MessageTypeWire::from_bytes(initial_bytes.as_slice()).unwrap();
            println!("message_type_struct: {:?}", message_type_struct);
            let (msg, remainder) = MessageDecoder::from_bytes(initial_bytes.as_slice()).unwrap();
            assert_eq!([msg.to_bytes(), remainder.to_vec()].concat(), initial_bytes);
        }
    }
}
