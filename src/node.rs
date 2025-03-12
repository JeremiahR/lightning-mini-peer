use bitcoin::secp256k1::PublicKey;

#[derive(Debug)]
pub struct Node {
    pub public_key: [u8; 33],
    pub ip_address: String,
    pub port: u16,
}

impl Node {
    pub fn from_str(node_str: &str) -> Option<Node> {
        let parts: Vec<&str> = node_str.split('@').collect();
        if parts.len() != 2 {
            return None;
        }
        let public_key = parts[0].to_string();
        let address = parts[1].to_string();
        let ip_address = address.split(':').next().unwrap().to_string();
        let port = match address.split(':').nth(1).unwrap().parse() {
            Ok(port) => port,
            Err(_) => return None,
        };
        let public_key = match hex::decode(public_key.clone())
            .unwrap()
            .as_slice()
            .try_into()
        {
            Ok(key) => key,
            Err(_) => return None,
        };
        Some(Node {
            public_key,
            ip_address,
            port,
        })
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
