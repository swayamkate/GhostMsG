# Security policy and threat model

## Status

GhostMesh is an experimental scaffold. It is not suitable for protecting sensitive or high-risk communications.

## Intended protections (future)

- Message confidentiality and integrity against network intermediaries.
- Peer authentication and protection against message tampering.
- Replay resistance and protocol downgrade defenses.
- Reduced exposure of sender/recipient relationships through optional privacy-preserving routing.

## Out of scope for the initial prototype

- Guaranteed anonymity against a global passive adversary.
- Protection from compromised endpoints, operating systems, or malicious client builds.
- Hiding all traffic volume, timing, or network participation.
- Guaranteed availability or censorship resistance.

## Engineering rules

- Use established, reviewed cryptographic protocols and maintained libraries.
- Never invent or modify cryptographic primitives.
- Avoid logging message bodies, secrets, private keys, or unnecessary peer identifiers.
- Validate all untrusted network input and enforce size/time limits.
- Treat relays and discovery services as potentially observable or malicious.
- Do not claim anonymity without a documented adversary model and independent review.

## Reporting a vulnerability

Please do not publish exploitable details before maintainers have had a reasonable opportunity to investigate. Until a private security contact is configured, open a GitHub Security Advisory for this repository where available.
