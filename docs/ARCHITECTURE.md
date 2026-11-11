# Architecture

## Principles

1. Separate content privacy from metadata privacy. End-to-end encryption protects payload contents; routing design and traffic shaping address who-talks-to-whom leakage.
2. Transport independence: protocol types must not depend on a particular network.
3. No home-grown cryptography. Use reviewed protocols and maintained implementations.
4. Explicit threat model. Document what each deployment can and cannot hide.
5. Fail closed. Authentication, version, and integrity failures must reject messages.

## Planned components

| Component | Responsibility | Initial direction |
|---|---|---|
| CLI/client | User commands and local UX | Rust CLI |
| protocol | Versioned envelopes and validation | Current workspace crate |
| crypto | Identity, session establishment, AEAD | Reviewed Noise-based design and maintained crates |
| transport | Peer connections and stream framing | libp2p evaluation |
| discovery | Peer discovery | Kademlia DHT; discovery is not anonymity |
| relay | Store/forward and relay policy | Minimal metadata, bounded queues |
| mixnet adapter | Optional anonymous message transport | Nym integration research |
| storage | Local encrypted state | SQLite after key/storage design review |

## Target message path

client -> authenticated encryption -> transport adapter -> optional mixnet -> recipient adapter -> authenticated decryption -> client

Direct P2P mode exposes network endpoints to peers and may expose them to observers. A DHT can reveal lookup and participation metadata. Neither should be described as anonymous.

## Metadata privacy work

A mixnet can make timing and sender-recipient correlation harder through batching, delays, and route mixing. It does not guarantee protection against every global passive observer, compromised nodes, endpoint malware, traffic confirmation, or application-layer identifiers. Cover traffic and padding are future design questions and must be measured for bandwidth and latency costs.

## Current implementation

The repository currently contains only the CLI scaffold and protocol envelope. No cryptographic or networking component is active.
