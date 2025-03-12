use std::collections::HashMap;

use bitcoin::secp256k1::SecretKey;

use crate::{
    config::DO_CONNECT_TO_NEW_NODES,
    message_decoder::MessageContainer,
    messages::{ChannelAnnouncementMessage, InitMessage, NodeAnnouncementMessage, PongMessage},
    node::Node,
    node_connection::{NodeConnection, NodeConnectionError},
    serialization::{PointElement, SerializableToBytes, ShortChannelIDElement},
};

#[allow(dead_code)]
#[derive(Debug)]
pub enum MessageHandlerError {
    NodeConnectionError(NodeConnectionError),
    NodeHandshakeError(NodeConnectionError),
}

pub struct MiniPeer {
    secret_key: SecretKey,
    node_connections: HashMap<[u8; 33], NodeConnection>,
    // eventually make a channel type not just the announcement message
    known_channels: HashMap<ShortChannelIDElement, ChannelAnnouncementMessage>,
    known_nodes: HashMap<PointElement, NodeAnnouncementMessage>,
}

impl MiniPeer {
    pub fn new(secret_key: SecretKey) -> Self {
        MiniPeer {
            secret_key,
            node_connections: HashMap::new(),
            known_channels: HashMap::new(),
            known_nodes: HashMap::new(),
        }
    }

    pub fn num_connections(&self) -> usize {
        self.node_connections.len()
    }

    pub async fn event_loop(&mut self) {
        loop {
            let mut inbounds = Vec::new();
            let mut disconnects = Vec::new();
            for node_conn in &mut self.node_connections.values_mut() {
                match node_conn.read_next_message().await {
                    Ok(wrapped_message) => {
                        inbounds.push((wrapped_message, node_conn.public_key.clone()));
                    }
                    Err(err) => {
                        match err {
                            NodeConnectionError::IOError(_) => {
                                disconnects.push(node_conn.public_key.clone());
                            }
                            _ => {
                                println!("Failed to read: {:?}", err);
                            }
                        }
                        continue;
                    }
                }
                if node_conn.ready_for_ping() {
                    node_conn.send_ping().await.unwrap();
                }
            }
            for node_public_key in disconnects {
                self.node_connections.remove(&node_public_key);
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
        println!("Connected to node: {}", node.address());
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
        node_public_key: [u8; 33],
    ) -> Result<(), MessageHandlerError> {
        println!("Received message: {:?}", wrapped);
        let node_conn = self.node_connections.get_mut(&node_public_key).unwrap();
        match wrapped {
            MessageContainer::Ping(ping) => {
                let pong = MessageContainer::Pong(PongMessage::from_ping(ping));
                match node_conn.encrypt_and_send_message(&pong).await {
                    Ok(_) => (),
                    Err(e) => return Err(MessageHandlerError::NodeConnectionError(e)),
                };
            }
            MessageContainer::NodeAnnouncement(announcement) => {
                if !self
                    .node_connections
                    .contains_key(&announcement.node_id.value)
                {
                    if !self.known_nodes.contains_key(&announcement.node_id) {
                        self.known_nodes
                            .insert(announcement.node_id.clone(), announcement.clone());
                        println!("Found new node: {:?}", announcement.node_id.clone());
                        println!("Known nodes: {}", self.known_nodes.len())
                    }
                    match announcement.as_node() {
                        Some(node) => {
                            println!("Found new node: {}", node.address());
                            if DO_CONNECT_TO_NEW_NODES {
                                self.open_node_connection(&node).await.unwrap();
                            } else {
                                println!(
                                   "Not connecting to new node because DO_CONNECT_TO_NEW_NODES=false."
                               );
                            }
                        }
                        None => {
                            println!("Found no address in node announcement");
                        }
                    }
                } else {
                    println!("Already connected to node.");
                }
            }
            MessageContainer::ChannelAnnouncement(msg) => {
                if !self
                    .known_channels
                    .contains_key(&msg.short_channel_id.clone())
                {
                    self.known_channels
                        .insert(msg.short_channel_id.clone(), msg.clone());
                    println!("Found new channel: {:?}", msg.short_channel_id.clone());
                    println!("Known channels: {}", self.known_channels.len())
                } else {
                }
            }
            MessageContainer::GossipTimestampFilter(gtf) => {
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
