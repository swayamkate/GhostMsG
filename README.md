# GhostMesh

**GhostMesh** is an experimental open-source project exploring privacy-first peer-to-peer messaging.

Its goal is to combine end-to-end encrypted message contents with network designs that reduce metadata exposure. It is a research prototype, not an anonymity guarantee or a production-ready secure messenger.

## Project status

Early scaffold (v0.1.0). The current code provides a Rust workspace, a CLI status command, and a versioned transport-independent message envelope. It does **not** yet provide encryption, peer-to-peer networking, anonymous routing, or secure identity storage.

## Design goals

- End-to-end confidentiality and peer authentication.
- Minimize metadata retained by clients and relays.
- Separate the message protocol from transport implementations.
- Support replaceable transports, including direct P2P and mixnet-backed paths.
- Make security assumptions and limitations explicit.
- Keep the implementation auditable and testable.

## Workspace

- apps/cli — developer-facing command-line client.
- crates/protocol — versioned message envelope types.

Planned modules are documented in docs/ARCHITECTURE.md.

## Build

Requires Rust and Cargo (stable toolchain).

    cargo fmt --all -- --check
    cargo test --workspace
    cargo run -p ghostmesh-cli -- status

## Security

Do not use GhostMesh to protect real-world sensitive communications yet. See docs/SECURITY.md for the current threat model and responsible disclosure guidance.

## Roadmap

See docs/ROADMAP.md.

## License

MIT. See LICENSE.
