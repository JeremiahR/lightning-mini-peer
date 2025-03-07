use std::collections::HashMap;

use bitcoin::secp256k1::SecretKey;

use crate::{
    message_decoder::MessageContainer,
    messages::{InitMessage, PongMessage},
    node::Node,
    node_connection::{NodeConnection, NodeConnectionError},
    serialization::BytesSerializable,
};

#[allow(dead_code)]
#[derive(Debug)]
pub enum MessageHandlerError {
    NodeConnectionError(NodeConnectionError),
    NodeHandshakeError(NodeConnectionError),
}

pub struct MiniPeer {
    secret_key: SecretKey,
    node_connections: HashMap<String, NodeConnection>,
}

impl MiniPeer {
    pub fn new(secret_key: SecretKey) -> Self {
        MiniPeer {
            secret_key,
            node_connections: HashMap::new(),
        }
    }

    pub async fn event_loop(&mut self) {
        // TODO: send pings progamatically
        loop {
            let mut inbounds = Vec::new();
            for node_conn in &mut self.node_connections.values_mut() {
                match node_conn.read_next_message().await {
                    Ok(wrapped_message) => {
                        println!("Received message: {:?}", wrapped_message);
                        inbounds.push((wrapped_message, node_conn.public_key.clone()));
                    }
                    Err(err) => {
                        println!("Failed to read: {:?}", err);
                        return;
                    }
                }
            }
            for (message, node_public_key) in inbounds {
                self.handle_inbound_message(message, node_public_key)
                    .await
                    .unwrap();
            }
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
    }

    pub async fn open_node_connection(&mut self, node: &Node) -> Result<(), MessageHandlerError> {
        let mut node_connection = match NodeConnection::new(node, self.secret_key).await {
            Ok(conn) => conn,
            Err(err) => {
                println!("Failed to create node connection: {:?}", err);
                return Err(MessageHandlerError::NodeConnectionError(err));
            }
        };
        match node_connection.handshake().await {
            Ok(_) => (),
            Err(err) => {
                println!("Failed to handshake: {:?}", err);
                return Err(MessageHandlerError::NodeHandshakeError(err));
            }
        };
        let init = b"\x00\x10\x00\x00\x00\x01\xaa";
        let (im, _) = InitMessage::from_bytes(init).unwrap();
        let wrapped = MessageContainer::Init(im);
        node_connection
            .encrypt_and_send_message(&wrapped)
            .await
            .unwrap();
        self.node_connections
            .insert(node.public_key.clone(), node_connection);
        Ok(())
    }

    pub async fn handle_inbound_message(
        &mut self,
        wrapped: MessageContainer,
        node_public_key: String,
    ) -> Result<(), MessageHandlerError> {
        println!("Received message: {:?}", wrapped);
        let node_conn = self.node_connections.get_mut(&node_public_key).unwrap();
        match wrapped {
            MessageContainer::Ping(ping) => {
                println!("Responding to ping.");
                let pong = MessageContainer::Pong(PongMessage::from_ping(ping));
                match node_conn.encrypt_and_send_message(&pong).await {
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
                match node_conn.encrypt_and_send_message(&response).await {
                    Ok(_) => (),
                    Err(e) => return Err(MessageHandlerError::NodeConnectionError(e)),
                };
            }
            _ => {}
        }
        Ok(())
    }
}
