use crate::messages::{
    ChannelAnnouncementMessage, InitMessage, MessageType, PingMessage, PongMessage,
};
use crate::wire::{BytesSerializable, MessageTypeWire};

#[derive(Debug)]
pub enum MessageDecoderError {
    Error,
}

#[derive(Debug)]
pub enum MessageContainer {
    Init(InitMessage),
    Ping(PingMessage),
    Pong(PongMessage),
    ChannelAnnouncement(ChannelAnnouncementMessage),
}

impl MessageContainer {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            MessageContainer::Init(message) => message.to_bytes(),
            MessageContainer::Ping(message) => message.to_bytes(),
            MessageContainer::Pong(message) => message.to_bytes(),
            MessageContainer::ChannelAnnouncement(message) => message.to_bytes(),
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
            _ => Err(MessageDecoderError::Error),
        }
    }
}
