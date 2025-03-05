use bitcoin::secp256k1::PublicKey;
use std::str::FromStr;

#[derive(Debug)]
pub struct Node {
    pub public_key: String,
    pub ip_address: String,
    pub port: u16,
}

impl Node {
    pub fn address(&self) -> String {
        format!("{}:{}", self.ip_address, self.port)
    }

    pub fn bitcoin_public_key(&self) -> PublicKey {
        PublicKey::from_str(&self.public_key).unwrap()
    }
}
