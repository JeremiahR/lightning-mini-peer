use crate::message_decoder::MessageContainer;

#[allow(dead_code)]
#[derive(Debug)]
pub enum MessageHandlerError {
    Error,
}

pub struct MessageHandler {}

impl MessageHandler {
    pub fn new() -> Self {
        MessageHandler {}
    }

    pub async fn handle_inbound(
        &self,
        wrapped: MessageContainer,
    ) -> Result<(), MessageHandlerError> {
        println!("Received message: {:?}", wrapped);
        Ok(())
    }
}
