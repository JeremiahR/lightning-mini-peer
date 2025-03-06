use crate::message_decoder::MessageContainer;
use crate::message_decoder::MessageDecoder;
use crate::vendor::KeysManager;
use bitcoin::secp256k1::PublicKey as BitcoinPublicKey;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::secp256k1::SecretKey;
use bitcoin::secp256k1::SignOnly;
use lightning::ln::msgs::LightningError;
use lightning::ln::peer_channel_encryptor::{MessageBuf, NextNoiseStep};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::node::Node;
use crate::util::new_random_secret_key;
use crate::vendor::PeerChannelEncryptor;
use std::sync::Arc;

#[allow(dead_code)]
#[derive(Debug)]
pub enum NodeConnectionError {
    HandshakeFailed,
    NoMessageFound,
    InvalidHeaderLength,
    DecryptionError(LightningError),
    ConnectionError(std::io::Error),
    IOError(std::io::Error),
    LightningError(LightningError),
    MessageDecodeError,
}

pub struct NodeConnection {
    stream: TcpStream,
    secp: Secp256k1<SignOnly>,
    peer_encryptor: PeerChannelEncryptor,
    km: Arc<KeysManager>,
}

impl NodeConnection {
    pub async fn new(node: &Node, node_secret_key: SecretKey) -> Result<Self, NodeConnectionError> {
        let ephemeral_key = new_random_secret_key();
        let stream = match TcpStream::connect(node.address()).await {
            Ok(stream) => stream,
            Err(err) => {
                println!("Failed to connect to {}: {}", node.address(), err);
                return Err(NodeConnectionError::ConnectionError(err));
            }
        };
        println!("Connected to {} @ {}", node.public_key, node.address());
        Ok(NodeConnection {
            stream,
            secp: Secp256k1::signing_only(),
            peer_encryptor: PeerChannelEncryptor::new_outbound(
                node.bitcoin_public_key().clone(),
                ephemeral_key,
            ),
            km: Arc::new(KeysManager::new(&node_secret_key.secret_bytes(), 0, 0)),
        })
    }

    async fn write_raw_data(&mut self, data: &[u8]) -> Result<(), NodeConnectionError> {
        match self.stream.write_all(data).await {
            Ok(_) => Ok(()),
            Err(err) => Err(NodeConnectionError::IOError(err)),
        }
    }

    async fn read_exact_n_bytes(
        &mut self,
        num_bytes: usize,
    ) -> Result<Vec<u8>, NodeConnectionError> {
        let mut buffer: Vec<u8> = vec![0; num_bytes as usize];
        match self.stream.read_exact(&mut buffer).await {
            Ok(n) => {
                let response = buffer[..n].to_vec();
                Ok(response)
            }
            Err(err) => Err(NodeConnectionError::IOError(err)),
        }
    }

    pub async fn handshake(&mut self) -> Result<BitcoinPublicKey, NodeConnectionError> {
        let act_one = self.peer_encryptor.get_act_one(&self.secp);
        match self.write_raw_data(&act_one).await {
            Ok(_) => (),
            Err(err) => return Err(err),
        }
        let act_two = self.read_exact_n_bytes(50).await?;
        let (act_three, public_key) = match self.peer_encryptor.process_act_two(&act_two, &self.km)
        {
            Ok((x, y)) => (x, y),
            Err(err) => return Err(NodeConnectionError::LightningError(err)),
        };
        assert_eq!(act_three.len(), 66);
        match self.write_raw_data(&act_three).await {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        match self.peer_encryptor.get_noise_step() {
            NextNoiseStep::NoiseComplete => println!("Handshake completed with {}", public_key),
            _ => return Err(NodeConnectionError::HandshakeFailed),
        }
        Ok(public_key)
    }

    async fn wait_for_message(&mut self) -> Result<(), NodeConnectionError> {
        match self.stream.readable().await {
            Ok(_) => Ok(()),
            Err(err) => Err(NodeConnectionError::IOError(err)),
        }
    }

    async fn read_next_message_bytes(&mut self) -> Result<Vec<u8>, NodeConnectionError> {
        let mut header = match self.read_exact_n_bytes(18).await {
            Ok(header) => header,
            Err(err) => return Err(err),
        };
        if header.len() != 18 {
            return Err(NodeConnectionError::InvalidHeaderLength);
        }
        match self.peer_encryptor.decrypt_message(header.as_mut()) {
            Ok(_) => (),
            Err(err) => return Err(NodeConnectionError::DecryptionError(err)),
        }
        let length = u16::from_be_bytes([header[0], header[1]]);
        let mut message = self.read_exact_n_bytes(length as usize + 16).await?;
        match self.peer_encryptor.decrypt_message(message.as_mut()) {
            Ok(_) => (),
            Err(err) => return Err(NodeConnectionError::DecryptionError(err)),
        }
        Ok(message)
    }

    pub async fn read_next_message(&mut self) -> Result<MessageContainer, NodeConnectionError> {
        self.wait_for_message().await?;
        let bytes = self.read_next_message_bytes().await?;
        if bytes.is_empty() {
            return Err(NodeConnectionError::NoMessageFound);
        }
        let (message, _bytes) = match MessageDecoder::from_bytes(bytes.as_slice()) {
            Ok(msg) => msg,
            Err(_) => return Err(NodeConnectionError::MessageDecodeError),
        };
        Ok(message)
    }

    pub async fn encrypt_and_send_bytes(
        &mut self,
        bytes: &[u8],
    ) -> Result<(), NodeConnectionError> {
        let buf = MessageBuf::from_encoded(bytes);
        let encrypted = self.peer_encryptor.encrypt_buffer(buf);
        self.write_raw_data(encrypted.as_slice()).await?;
        Ok(())
    }

    pub async fn encrypt_and_send_message(
        &mut self,
        message: &MessageContainer,
    ) -> Result<(), NodeConnectionError> {
        let bytes = message.to_bytes();
        self.encrypt_and_send_bytes(bytes.as_slice()).await?;
        println!("Sent message {:?}", message);
        Ok(())
    }
}
