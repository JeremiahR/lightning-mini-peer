use crate::serialization::MessageTypeElement;
use crate::serialization::SerializedElement;

pub struct Message {
    msg_type: u16,
    name: String,
}

impl Message {
    fn type_to_structure(msg_type: u16) -> String {
        match msg_type {
            16 => "init".to_string(),
            _ => "unknown".to_string(),
        }
    }

    pub fn message_from_bytes(bytes: &[u8]) -> Message {
        let (m, bytes) = MessageTypeElement::from_bytes(bytes).unwrap();
        println!("message type: {:?}", m);
        Message {
            msg_type: m.id,
            name: m.name,
        }
    }
}

mod tests {
    use super::Message;

    #[test]
    fn test_decode_bytes() {
        let bytes = hex::decode("001000021100000708a0880a8a59a1012006226e46111a0b59caaf126043eb5bbf28c34f3a5e332a1fc7b2b73cf188910f2d7ef99482067a1b72fe9e411d37be8c").unwrap();
        let msg = Message::message_from_bytes(&bytes);
        assert_eq!(msg.msg_type, 16);
        assert_eq!(msg.name, "init");
    }
}
