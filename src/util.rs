use crate::node::Node;
use bitcoin::secp256k1::SecretKey as BitcoinSecretKey;
use secp256k1::rand::rngs::OsRng;
use secp256k1::Secp256k1;

pub fn parse_node(node_str: &str) -> Node {
    let parts: Vec<&str> = node_str.split('@').collect();
    let public_key = parts[0].to_string();
    let address = parts[1].to_string();
    let ip_address = address.split(':').next().unwrap().to_string();
    let port = address.split(':').nth(1).unwrap().parse().unwrap();
    Node {
        public_key,
        ip_address,
        port,
    }
}

pub fn new_random_secret_key() -> BitcoinSecretKey {
    let secp = Secp256k1::new();
    let (secret_key, _) = secp.generate_keypair(&mut OsRng);
    BitcoinSecretKey::from_slice(&secret_key.secret_bytes()).unwrap()
}
