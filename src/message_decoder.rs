use crate::messages::{
    ChannelAnnouncementMessage, InitMessage, MessageType, PingMessage, PongMessage, UnknownMessage,
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
    Unknown(UnknownMessage),
}

impl MessageContainer {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            MessageContainer::Init(message) => message.to_bytes(),
            MessageContainer::Ping(message) => message.to_bytes(),
            MessageContainer::Pong(message) => message.to_bytes(),
            MessageContainer::ChannelAnnouncement(message) => message.to_bytes(),
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
