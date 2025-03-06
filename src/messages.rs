// use crate::wire::{LightningType, WireFormatMessage};

// enum ParsingError {
//     InvalidMessage,
//     InvalidFeature,
// }

// #[derive(Debug, Clone, PartialEq, Eq)]
// struct InitMessage {
//     global_features: Vec<u8>,
//     local_features: Vec<u8>,
// }

// impl WireSerializable for InitMessage {
//     fn from_bytes(wire_message: &WireFormatMessage) -> Result<Self, ParsingError> {
//         let message_type = wire_message.get_element(LightningType::MessageType).unwrap();
//         if  != MessageType::Init.0 {
//             return Err(ParsingError::InvalidMessage);
//         }

//         let global_features = wire_message.payload[0..2].to_vec();
//         let local_features = wire_message.payload[2..4].to_vec();

//         Ok(InitMessage {
//             global_features,
//             local_features,
//         })
//     }

//     fn to_wire_format(&self) -> WireFormatMessage {
//         let mut payload = Vec::new();
//         payload.extend_from_slice(&self.global_features);
//         payload.extend_from_slice(&self.local_features);

//         WireFormatMessage {
//             type_id: 0,
//             payload,
//         }
//     }
// }
