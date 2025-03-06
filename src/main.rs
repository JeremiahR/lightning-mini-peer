use message_decoder::MessageContainer;
use message_handler::MessageHandler;
use messages::InitMessage;
use messages::PingMessage;
use wire::BytesSerializable;

use crate::util::new_random_secret_key;
use crate::util::parse_node;

use crate::node_connection::NodeConnection;
use std::env;

mod message_decoder;
mod message_handler;
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
    {
        let init = b"\x00\x10\x00\x00\x00\x01\xaa";
        let (im, _) = InitMessage::from_bytes(init).unwrap();
        let wrapped = MessageContainer::Init(im);
        node_conn.encrypt_and_send_message(&wrapped).await.unwrap();
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
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    {
        let ping = PingMessage {
            num_pong_bytes: 10,
            ignored: vec![0; 10],
        };
        let wrapped = MessageContainer::Ping(ping);
        node_conn.encrypt_and_send_message(&wrapped).await.unwrap();
    }
    let message_handler = MessageHandler::new();
    loop {
        match node_conn.read_next_message().await {
            Ok(msg) => {
                message_handler.handle_inbound(msg).await.unwrap();
            }
            Err(err) => {
                println!("Failed to read: {:?}", err);
                break;
            }
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}
