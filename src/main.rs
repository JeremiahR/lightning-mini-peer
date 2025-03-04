use std::env;
// use lightning::ln::peer_channel_encryptor::PeerChannelEncryptor;

#[derive(Debug)]
struct Node {
    public_key: String,
    ip_address: String,
    port: u16,
}

fn parse_node(node_str: &str) -> Node {
    let parts: Vec<&str> = node_str.split('@').collect();
    let public_key = parts[0].to_string();
    let address = parts[1].to_string();
    let ip_address = address.split(':').next().unwrap().to_string();
    let port = address.split(':').nth(1).unwrap().parse().unwrap();
    Node {
        public_key,
        ip_address,
        port,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let node_str = args.last().unwrap();
    let node = parse_node(node_str);
    println!("Arguments: {:?}", node);
}
