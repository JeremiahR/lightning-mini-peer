// all dependencies and hacks from rust-lightning go here

use bitcoin::secp256k1;
use bitcoin::secp256k1::{PublicKey as BitcoinPublicKey, Secp256k1, SecretKey as BitcoinSecretKey};
pub use lightning::ln::peer_channel_encryptor::PeerChannelEncryptor;
pub use lightning::sign::KeysManager;

pub struct MiniPeerConnection {
    secp_ctx: Secp256k1<secp256k1::SignOnly>,
    ephemeral_key: BitcoinSecretKey,
}

impl MiniPeerConnection {
    pub fn new(secp_ctx: Secp256k1<secp256k1::SignOnly>, ephemeral_key: BitcoinSecretKey) -> Self {
        MiniPeerConnection {
            secp_ctx,
            ephemeral_key,
        }
    }

    pub fn new_peer_connector(&self, their_public_key: BitcoinPublicKey) -> PeerChannelEncryptor {
        PeerChannelEncryptor::new_outbound(their_public_key.clone(), self.ephemeral_key.clone())
    }
}
