use crate::util::new_random_secret_key;
use crate::util::parse_node;
use crate::vendor::KeysManager;
use crate::vendor::MiniPeerConnection;
use bitcoin::secp256k1::PublicKey as BitcoinPublicKey;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::secp256k1::SecretKey;
use hex;
use std::sync::Arc;

use crate::vendor::PeerChannelEncryptor;
use node::Node;
use std::env;
use std::str::FromStr;

mod node;
mod util;
mod vendor;

use std::io::{Read, Write};
use std::net::TcpStream;

struct NodeConnection {
    node_public_key: BitcoinPublicKey,
    stream: TcpStream,
    peer_encryptor: PeerChannelEncryptor,
}

fn handshake(node: &Node, node_secret_key: SecretKey) -> NodeConnection {
    let address = format!("{}:{}", node.ip_address, node.port);
    let secp_ctx = Secp256k1::signing_only();
    let remote_public_key = BitcoinPublicKey::from_str(&node.public_key).unwrap();
    let mut km = Arc::new(KeysManager::new(&node_secret_key.secret_bytes(), 0, 0));
    let mp = MiniPeerConnection::new(secp_ctx.clone(), new_random_secret_key());
    let mut peer_encryptor = mp.new_peer_connector(remote_public_key);
    let mut stream = TcpStream::connect(&address).unwrap();

    // let next_step = peer_encryptor.get_next_step();
    // println!("Next step: {:?}", next_step);
    println!("Connected to {}", address);
    let act_one = peer_encryptor.get_act_one(&secp_ctx);
    println!("Act One: {:?}", hex::encode(act_one));
    stream.write_all(act_one.as_ref()).unwrap();

    let mut buffer = [0; 50];
    let n = stream.read(&mut buffer).unwrap();
    println!("Received: {}", hex::encode(&buffer[..n]));
    let (act_three, public_key) = peer_encryptor
        .process_act_two(&buffer[..n], &mut km)
        .unwrap();
    println!("Act Three: {:?}", hex::encode(act_three));
    println!("Public Key: {:?}", public_key.to_string());
    stream.write_all(act_three.as_ref()).unwrap();

    let mut buffer = [0; 66];
    let n = stream.read(&mut buffer).unwrap();
    let node_public_key = peer_encryptor.process_act_three(&buffer[..n]).unwrap();

    NodeConnection {
        node_public_key,
        stream,
        peer_encryptor,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let node_str = args.last().unwrap();
    let node = parse_node(node_str);
    let node_secret_key = new_random_secret_key();
    // let node_public_key = node_secret_key.public_key(&Secp256k1::signing_only());

    let secp_ctx = Secp256k1::signing_only();

    let mut node_conn = handshake(&node, node_secret_key);

    println!("Arguments: {:?}", node);

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
