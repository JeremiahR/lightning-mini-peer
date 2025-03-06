// all dependencies and hacks from rust-lightning go here

pub use lightning::ln::msgs::LightningError;
pub use lightning::ln::peer_channel_encryptor::PeerChannelEncryptor;
pub use lightning::ln::peer_channel_encryptor::{MessageBuf, NextNoiseStep};
pub use lightning::sign::KeysManager;
