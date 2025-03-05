use crate::util::new_random_secret_key;
use crate::util::parse_node;

use crate::node_connection::NodeConnection;
use std::env;

mod node;
mod node_connection;
mod util;
mod vendor;

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
    match node_conn.get_next_message().await {
        Ok(_) => (),
        Err(err) => {
            println!("Failed to read: {:?}", err);
            return;
        }
    }

    // let mut buffer = [0; 512];
    // let n = stream.read(&mut buffer).unwrap();

    // peer_encryptor.decrypt_message(&mut buffer[..n]).unwrap();
    // println!("Received: {}", hex::encode(&buffer[..n]));
    // let init = b"\x00\x10\x00\x00\x00\x01\xaa";
    // stream.write_all(init).unwrap();
    // // now wait for the response
    // let mut buffer = [0; 512];
    // let n = stream.read(&mut buffer).unwrap();
    // // make a mutable copy of the message
    // peer_encryptor.decrypt_message(&mut buffer[..n]).unwrap();
    // println!("Received: {}", hex::encode(&buffer[..n]));
}

// test
mod test {
    use super::*;

    #[test]
    fn test_decode_message() {
        let bytes = hex::decode("19d915996c6dd5c8be418de4b5762d0e7d895e28e79dcb3d9421d3bcd2d9e94c6bc6df5ef7ea6c40056c3caf0fe46b6cfb30a6db6987929c37f5f80fcc74a2ccc2a77d9c6c484abd0a508411e21b9a7e8118b8").unwrap();
        // decode the first two bytes as a big-endian int
        let mst_type = u16::from_be_bytes([bytes[0], bytes[1]]);
        assert_eq!(mst_type, 1);
        // assert_eq!(bytes, "Hello, world!");
    }
}
