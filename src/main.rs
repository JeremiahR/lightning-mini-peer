use message_decoder::MessageDecoder;
use messages::PingMessage;

use crate::util::new_random_secret_key;
use crate::util::parse_node;

use crate::node_connection::NodeConnection;
use std::env;

mod message_decoder;
mod messages;
mod node;
mod node_connection;
mod util;
mod vendor;
mod wire;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let node_str = args.last().unwrap();
    let node = parse_node(node_str);
    let node_secret_key = new_random_secret_key();

    let mut node_conn = match NodeConnection::new(&node, node_secret_key).await {
        Ok(conn) => conn,
        Err(err) => {
            println!("Failed to create node connection: {:?}", err);
            return;
        }
    };
    match node_conn.handshake().await {
        Ok(_) => (),
        Err(err) => {
            println!("Failed to handshake: {:?}", err);
            return;
        }
    };
    match node_conn.send_init().await {
        Ok(_) => (),
        Err(err) => {
            println!("Failed to send init: {:?}", err);
            return;
        }
    }
    match node_conn.read_next_message().await {
        Ok(res) => {
            println!("Received message: {:?}", res);
        }
        Err(err) => {
            println!("Failed to read: {:?}", err);
            return;
        }
    }
    let _ping = PingMessage {
        num_pong_bytes: 10,
        ignored: vec![0; 10],
    };
    // match node_conn.wri(ping).await {
    //     Ok(_) => (),
    //     Err(err) => {
    //         println!("Failed to send ping: {:?}", err);
    //         return;
    //     }
    // }
}
