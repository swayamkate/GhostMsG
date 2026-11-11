# Roadmap

## v0.1 — Foundation
- [x] Rust workspace and CLI scaffold
- [x] Versioned transport-independent envelope
- [x] Initial architecture and threat-model documentation
- [ ] CI checks and baseline test coverage

## v0.2 — Protocol and identity design
- [ ] Specify canonical serialization and strict size limits
- [ ] Select a reviewed identity and session-establishment design
- [ ] Add key lifecycle and local storage threat model
- [ ] Add replay and downgrade test vectors

## v0.3 — Local encrypted messaging
- [ ] Implement authenticated encryption using maintained libraries
- [ ] Add two-process local integration tests
- [ ] Add safe error handling and redacted diagnostics

## v0.4 — P2P transport
- [ ] Evaluate and pin libp2p components
- [ ] Add authenticated peer connections
- [ ] Add bounded message framing and resource limits

## v0.5 — Relay and discovery
- [ ] Add opt-in relay mode
- [ ] Evaluate Kademlia discovery metadata exposure
- [ ] Add abuse controls and relay observability minimization

## v0.6 — Metadata privacy research
- [ ] Evaluate Nym mixnet integration
- [ ] Document latency, bandwidth, and adversary trade-offs
- [ ] Add privacy-focused integration tests

## v1.0 — Security review
- [ ] Independent protocol and implementation review
- [ ] Reproducible builds and dependency audit
- [ ] Publish a deployment-specific threat model
- [ ] Remove unsupported anonymity claims
