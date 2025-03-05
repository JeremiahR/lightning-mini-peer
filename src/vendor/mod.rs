// all dependencies and hacks from rust-lightning go here

use bitcoin::secp256k1;
use bitcoin::secp256k1::{PublicKey as BitcoinPublicKey, Secp256k1, SecretKey as BitcoinSecretKey};
pub use lightning::ln::peer_channel_encryptor::PeerChannelEncryptor;
pub use lightning::sign::KeysManager;
