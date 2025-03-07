use peer::MiniPeer;

use crate::util::new_random_secret_key;
use crate::util::parse_node;

use std::env;

mod base_types;
mod message_decoder;
mod messages;
mod node;
mod node_connection;
mod peer;
mod util;
mod vendor;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let node_str = args.last().unwrap();
    let node = parse_node(node_str);
    let session_secret_key = new_random_secret_key();

    let mut peer = MiniPeer::new(session_secret_key);
    peer.open_node_connection(&node).await.unwrap();
    peer.event_loop().await;
}
