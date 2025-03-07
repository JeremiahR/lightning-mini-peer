use node::Node;
use peer::MiniPeer;

use crate::util::new_random_secret_key;

use std::env;

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
    let args: Vec<String> = env::args().collect();
    let node_str = args.last().unwrap();
    let node = Node::from_str(node_str);
    let session_secret_key = new_random_secret_key();

    let mut peer = MiniPeer::new(session_secret_key);
    peer.open_node_connection(&node).await.unwrap();
    peer.event_loop().await;
}
