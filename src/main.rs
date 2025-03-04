use crate::util::new_random_secret_key;
use crate::util::parse_node;
use crate::vendor::MiniPeerConnection;
use bitcoin::secp256k1::PublicKey as BitcoinPublicKey;
use bitcoin::secp256k1::Secp256k1;

use std::env;
use std::str::FromStr;

mod node;
mod util;
mod vendor;

fn main() {
    let args: Vec<String> = env::args().collect();
    let node_str = args.last().unwrap();
    let node = parse_node(node_str);

    let their_public_key = BitcoinPublicKey::from_str(&node.public_key).unwrap();
    let secp_ctx = Secp256k1::signing_only();
    let mp = MiniPeerConnection::new(secp_ctx, new_random_secret_key());
    let res = mp.new_outbound_connection(their_public_key);
    println!("Arguments: {:?}", node);
    println!("Result: {:?}", res);
}
