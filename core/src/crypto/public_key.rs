//! Public key types

use crate::schema;

/// Public keys
#[derive(Copy, Clone, Debug)]
pub enum PublicKey {
    /// ECDSA public keys
    #[cfg(feature = "ecdsa")]
    Ecdsa(EcdsaKey),

    /// Ed25519 public keys
    // TODO(tarcieri): use ed25519-dalek's `PublicKey` type
    Ed25519([u8; 32]),
}

// TODO(tarcieri): this should eventually be a `TryFrom`
impl From<schema::public_key::PublicKey> for PublicKey {
    fn from(key: schema::public_key::PublicKey) -> PublicKey {
        match key {
            schema::public_key::PublicKey::Ed25519(bytes) => PublicKey::Ed25519(bytes),
        }
    }
}

/// ECDSA public keys
#[derive(Copy, Clone, Debug)]
pub enum EcdsaKey {
    /// NIST P-256 public keys
    #[cfg(feature = "ecdsa")]
    P256(ecdsa::curve::nistp256::PublicKey),
}
