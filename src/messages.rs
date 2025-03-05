use crate::message_types::MessageTypes;
use std::collections::HashMap;
use strum::IntoEnumIterator;

struct Message {
    msg_type: u16,
    name: String,
}

impl Message {
    // it's lazy to do this every time, but it can be optimized later
    fn enum_name_lookup() -> HashMap<i32, String> {
        let mut map = HashMap::new();
        for variant in MessageTypes::iter() {
            let name: &str = variant.clone().into();
            let name = name.to_lowercase();
            map.insert(variant.clone() as i32, name);
        }
        map
    }

    pub fn message_from_bytes(bytes: &[u8]) -> Message {
        let msg_type = u16::from_be_bytes([bytes[0], bytes[1]]);
        let lookup = Self::enum_name_lookup();
        let name = match lookup.get(&(msg_type as i32)) {
            Some(name) => name.to_string(),
            None => "unknown".to_string(),
        };
        println!("message type: {}", msg_type);
        Message { msg_type, name }
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
