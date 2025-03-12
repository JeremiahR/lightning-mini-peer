use node::Node;
use peer::MiniPeer;

use crate::util::new_random_secret_key;

use std::env;

mod config;
mod message_decoder;
mod messages;
mod node;
mod node_connection;
mod peer;
mod serialization;
mod util;
mod vendor;

#[tokio::main]
async fn main() {
    let mut peer = MiniPeer::new(new_random_secret_key());

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: lmprs2 <node_address_1> ... <node_address_n>");
        return;
    }

    let mut nodes = Vec::new();
    for arg in args.iter().skip(1) {
        let node_str = arg;
        let node = match Node::from_str(node_str) {
            Some(node) => node,
            None => {
                eprintln!("Error parsing node address: {}", arg);
                continue;
            }
        };
        nodes.push(node);
    }

    println!("Attempting to connect to {} nodes", nodes.len());
    for node in &nodes {
        match peer.open_node_connection(&node).await {
            Ok(()) => println!("Connected to node {:?}", node.display_str()),
            Err(e) => eprintln!("Error connecting to node {:?}: {:?}", node.display_str(), e),
        }
    }
    if peer.num_connections() == nodes.len() {
        println!("Successfully connected to all nodes");
    }
    if peer.num_connections() > 0 {
        println!("Connected to {} nodes", peer.num_connections());
        peer.event_loop().await;
    } else {
        println!("Failed to connect to any nodes");
    }
}
