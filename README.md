# Lightning Mini Peer

A tiny lightning network peer for mapping and sniffing the network.

Goals:

1. You should be able to pass in one node address and have it index every reachable node.
2. It should be able to connect to N nodes and stream JSON-formatted messages to stdout or a file for analysis.

# Usage

Setup a lightning network in Polar. Copy the "P2P External" address of one node, it looks like this "02689e38c2b9a9142ced61c080165dd724456970c3fa2ef09fc042149d85892bd5@127.0.0.1:9839"

Run `cargo run <nodeid>`

Right now all it does is handshake with the node. More features being added now.
