use crate::{
    message_decoder::MessageContainer,
    messages::PongMessage,
    node_connection::{NodeConnection, NodeConnectionError},
};

#[allow(dead_code)]
#[derive(Debug)]
pub enum MessageHandlerError {
    NodeConnectionError(NodeConnectionError),
}

pub struct MessageHandler {}

impl MessageHandler {
    pub fn new() -> Self {
        MessageHandler {}
    }

    pub async fn handle_inbound(
        &self,
        wrapped: MessageContainer,
        conn: &mut NodeConnection,
    ) -> Result<(), MessageHandlerError> {
        println!("Received message: {:?}", wrapped);
        match wrapped {
            MessageContainer::Ping(ping) => {
                println!("Responding to ping.");
                let pong = MessageContainer::Pong(PongMessage::from_ping(ping));
                match conn.encrypt_and_send_message(&pong).await {
                    Ok(_) => (),
                    Err(e) => return Err(MessageHandlerError::NodeConnectionError(e)),
                };
            }
            MessageContainer::GossipTimestampFilter(gtf) => {
                println!("Responding to gossip timestamp filter.");
                let mut our_filter = gtf.clone();
                // we ask for all the gossip
                our_filter.first_timestamp = 0;
                let response = MessageContainer::GossipTimestampFilter(our_filter);
                match conn.encrypt_and_send_message(&response).await {
                    Ok(_) => (),
                    Err(e) => return Err(MessageHandlerError::NodeConnectionError(e)),
                };
            }
            _ => {
                // println!("Received but not handling message: {:?}", wrapped);
            }
        }
        Ok(())
    }
}
