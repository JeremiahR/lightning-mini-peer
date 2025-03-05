use crate::vendor::KeysManager;
use bitcoin::secp256k1::PublicKey as BitcoinPublicKey;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::secp256k1::SecretKey;
use bitcoin::secp256k1::SignOnly;
use lightning::ln::peer_channel_encryptor::NextNoiseStep;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::node::Node;
use crate::util::new_random_secret_key;
use crate::vendor::PeerChannelEncryptor;
use hex;
use std::sync::Arc;

#[derive(Debug)]
pub enum NodeConnectionError {
    SocketError,
}

pub struct NodeConnection {
    node_public_key: BitcoinPublicKey,
    stream: TcpStream,
    secp: Secp256k1<SignOnly>,
    ephemeral_key: SecretKey,
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
                return Err(NodeConnectionError::SocketError);
            }
        };
        println!("Connected to {}", node.address());
        Ok(NodeConnection {
            node_public_key: node.bitcoin_public_key(),
            stream,
            secp: Secp256k1::signing_only(),
            ephemeral_key,
            peer_encryptor: PeerChannelEncryptor::new_outbound(
                node.bitcoin_public_key().clone(),
                ephemeral_key,
            ),
            km: Arc::new(KeysManager::new(&node_secret_key.secret_bytes(), 0, 0)),
        })
    }

    async fn write_all(&mut self, data: &[u8]) -> Result<(), NodeConnectionError> {
        match self.stream.write_all(data).await {
            Ok(_) => {
                println!("Wrote {:?}", hex::encode(data));
                Ok(())
            }
            Err(err) => {
                println!("Failed to write data: {}", err);
                Err(NodeConnectionError::SocketError)
            }
        }
    }

    async fn read_n_bytes(&mut self, num_bytes: usize) -> Result<Vec<u8>, NodeConnectionError> {
        let mut buffer: Vec<u8> = vec![0; num_bytes as usize];
        match self.stream.read(&mut buffer).await {
            Ok(n) => {
                let response = buffer[..n].to_vec();
                println!("Read: {:?}", hex::encode(&response));
                Ok(response)
            }
            Err(err) => {
                println!("Failed to receive act one: {:?}", err);
                Err(NodeConnectionError::SocketError)
            }
        }
    }

    async fn send_act_one(&mut self) -> Result<(), NodeConnectionError> {
        let act_one = self.peer_encryptor.get_act_one(&self.secp);
        match self.write_all(&act_one).await {
            Ok(_) => Ok(()),
            Err(err) => {
                println!("Failed to send act one: {:?}", err);
                Err(NodeConnectionError::SocketError)
            }
        }
    }

    async fn process_act_two(
        &mut self,
        act_two: Vec<u8>,
    ) -> Result<(BitcoinPublicKey), NodeConnectionError> {
        match self.peer_encryptor.process_act_two(&act_two, &self.km) {
            Ok((act_three, public_key)) => match self.write_all(&act_three).await {
                Ok(_) => Ok(public_key),
                Err(err) => {
                    println!("Failed to send act three: {:?}", err);
                    Err(NodeConnectionError::SocketError)
                }
            },
            Err(err) => {
                println!("Failed to process act two: {:?}", err);
                Err(NodeConnectionError::SocketError)
            }
        }
    }

    async fn print_noise_state(&self) {
        let state = match self.peer_encryptor.get_noise_step() {
            NextNoiseStep::ActOne => "Act One",
            NextNoiseStep::ActTwo => "Act Two",
            NextNoiseStep::ActThree => "Act Three",
            NextNoiseStep::NoiseComplete => "Noise Complete",
        };
        println!("Noise state: {}", state);
    }

    pub async fn handshake(&mut self) -> Result<BitcoinPublicKey, NodeConnectionError> {
        self.send_act_one().await?;
        let act_two = self.read_n_bytes(66).await?;
        let public_key = self.process_act_two(act_two).await?;
        self.print_noise_state().await;
        Ok(public_key)
    }

    pub async fn send_init(&mut self) -> Result<(), NodeConnectionError> {
        let mut init = b"\x00\x10\x00\x00\x00\x01\xaa";
        match self.write_all(init).await {
            Ok(_) => {
                println!("sent init");
                Ok(())
            }
            Err(err) => {
                println!("Failed to send init: {:?}", err);
                Err(NodeConnectionError::SocketError)
            }
        }
    }

    pub async fn get_next_message(&mut self) -> Result<Vec<u8>, NodeConnectionError> {
        let mut header = self.read_n_bytes(18).await?;
        self.peer_encryptor
            .decrypt_message(header.as_mut())
            .unwrap();
        println!("decrypted header: {:?}", hex::encode(&header));
        let length = u16::from_be_bytes([header[0], header[1]]);
        println!("message length: {}", length);
        let mut message = self.read_n_bytes(length as usize + 16).await?;
        self.peer_encryptor
            .decrypt_message(message.as_mut())
            .unwrap();
        println!("decrypted message: {:?}", hex::encode(&message));
        Ok(message)
    }
}
