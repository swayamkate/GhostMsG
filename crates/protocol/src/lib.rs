//! Versioned, transport-independent GhostMesh message types.
//! This crate contains no networking or cryptographic primitives.

use serde::{Deserialize, Serialize};

/// Initial wire-schema version. Breaking changes require a version increment.
pub const PROTOCOL_VERSION: u16 = 1;

/// A transport-independent message envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageEnvelope {
    pub protocol_version: u16,
    pub message_id: String,
    pub recipient: String,
    pub payload: Vec<u8>,
}

impl MessageEnvelope {
    /// Construct an envelope using the current protocol version.
    pub fn new(message_id: String, recipient: String, payload: Vec<u8>) -> Self {
        Self { protocol_version: PROTOCOL_VERSION, message_id, recipient, payload }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn envelope_uses_current_protocol_version() {
        let envelope = MessageEnvelope::new("msg-1".into(), "peer-1".into(), vec![1, 2, 3]);
        assert_eq!(envelope.protocol_version, PROTOCOL_VERSION);
        assert_eq!(envelope.payload, vec![1, 2, 3]);
    }
}
