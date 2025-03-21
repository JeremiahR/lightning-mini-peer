use bitcoin::secp256k1::SecretKey as BitcoinSecretKey;
use secp256k1::rand::rngs::OsRng;
use secp256k1::Secp256k1;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn new_random_secret_key() -> BitcoinSecretKey {
    let secp = Secp256k1::new();
    let (secret_key, _) = secp.generate_keypair(&mut OsRng);
    BitcoinSecretKey::from_slice(&secret_key.secret_bytes()).unwrap()
}

pub fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}
