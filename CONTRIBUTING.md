# Contributing

Thanks for helping build GhostMesh. This is experimental security software; correctness, clarity, and conservative claims matter more than feature count.

## Before opening a pull request

1. Open or find an issue describing the change.
2. Keep protocol changes versioned and documented.
3. Never implement cryptographic primitives from scratch.
4. Add tests for behavior and failure cases.
5. Run cargo fmt --all -- --check, cargo clippy --workspace --all-targets -- -D warnings, and cargo test --workspace.
6. Explain security implications and new trust assumptions.

## Security-sensitive changes

Do not include secrets, private keys, real user traffic, or exploit instructions targeting third-party systems. Report suspected vulnerabilities privately using the process in docs/SECURITY.md.
