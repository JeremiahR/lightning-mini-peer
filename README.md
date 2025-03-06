# Lightning Mini Peer

A tiny lightning network peer for mapping and sniffing the network.

Goals:

1. You should be able to pass in one node address and have it index every reachable node.
2. It should be able to connect to N nodes and stream JSON-formatted messages to stdout or a file for analysis.

# Usage

Setup a lightning network in Polar. Copy the "P2P External" address of one node, it looks like this "02689e38c2b9a9142ced61c080165dd724456970c3fa2ef09fc042149d85892bd5@127.0.0.1:9839"

Run `cargo run <nodeid>`

See below for the features that are implemented.

# Todos

- [DONE] De/serialize pings, pongs, inits, and basic message types.
- [DONE] Handshake and keep alive connection. Print inbound/outbound messages.
- [DONE] Send pings, recieve pongs.
- Create message handler that responds to pings.
- De/serialize gossip and channel messages.
- Interrogate connected node(s) to learn about more nodes.
- Interrogate connected node(s) to learn about channels.

# Spec

- What are we building?
    > Lightning "minipeer" like the bitcoin one (use library for noise protocol), implement as many P2P exchanges as you can
- Per Carla:
    > LDK does a whole lot under the hood that you don't want for a node like this - connecting to a bitcoin node, validating UTXOs etc.
    > For this project we're essentially thinking about the most lightweight, low effort shell of a lightning node that one could feasibly put up.
    > For example, if you're a researcher who'e interested in figuring out the parameters for minisketch gossip in lightning - you could just spin up a few of these and record whatever you see on the gossip network without having to bother with a bitcoin node at all.
    > You'll def want to re-use components like the bolt-8 handshake, because that's as lightweight as is gets already, but for the other protocol messages there's a lot that can be cut out.
