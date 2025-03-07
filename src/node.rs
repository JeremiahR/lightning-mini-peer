use bitcoin::secp256k1::PublicKey;
use std::str::FromStr;

#[derive(Debug)]
pub struct Node {
    pub public_key: [u8; 33],
    pub ip_address: String,
    pub port: u16,
}

impl Node {
    pub fn from_str(node_str: &str) -> Node {
        let parts: Vec<&str> = node_str.split('@').collect();
        let public_key = parts[0].to_string();
        let address = parts[1].to_string();
        let ip_address = address.split(':').next().unwrap().to_string();
        let port = address.split(':').nth(1).unwrap().parse().unwrap();
        let public_key = hex::decode(public_key.clone())
            .unwrap()
            .as_slice()
            .try_into()
            .unwrap();
        Node {
            public_key,
            ip_address,
            port,
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.ip_address, self.port)
    }

    pub fn bitcoin_public_key(&self) -> PublicKey {
        PublicKey::from_slice(&self.public_key).unwrap()
    }

    pub fn display_str(&self) -> String {
        format!("{}@{}", hex::encode(self.public_key), self.address())
    }
}
