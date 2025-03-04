// all dependencies and hacks from rust-lightning go here

use bitcoin::secp256k1;
use bitcoin::secp256k1::{PublicKey as BitcoinPublicKey, Secp256k1, SecretKey as BitcoinSecretKey};
pub use lightning::ln::peer_channel_encryptor::PeerChannelEncryptor;

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

    pub fn new_outbound_connection(&self, their_public_key: BitcoinPublicKey) -> Vec<u8> {
        let mut peer_encryptor = PeerChannelEncryptor::new_outbound(
            their_public_key.clone(),
            self.ephemeral_key.clone(),
        );
        let res = peer_encryptor.get_act_one(&self.secp_ctx).to_vec();
        res
    }
}
