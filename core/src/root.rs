//! Root configuration: controls sensitive administrative authority
//!
//! Inspired by the role of the same name in The Update Framework.
//! See Section 4.3 of the TUF spec:
//!
//! <https://github.com/theupdateframework/specification/blob/master/tuf-spec.md#4-document-formats>

use crate::{crypto::PublicKey, error::Error, schema::provision::Uuid};
use heapless::Vec;

/// Maximum number of keys allowed for root role
pub(crate) type MaxKeys = heapless::consts::U8;

/// Root configuration: controls sensitive administrative authority
// TODO(tarcieri): extract type for threshold key sets
#[derive(Debug, Default)]
pub(crate) struct Root {
    /// Threshold for number of keys required to perform a root action
    threshold: usize,

    /// Public keys for the root role
    public_keys: Vec<PublicKey, MaxKeys>,
}

impl Root {
    /// Create new [`Root`] configuration
    pub fn new(threshold: usize, keys: impl IntoIterator<Item = PublicKey>) -> Result<Self, Error> {
        let mut public_keys = Vec::new();

        for key in keys.into_iter() {
            public_keys.push(key).map_err(|_| Error::Threshold)?;
        }

        if threshold < 1 || threshold > public_keys.len() {
            return Err(Error::Threshold);
        }

        Ok(Root {
            threshold,
            public_keys,
        })
    }

    /// Is the [`Root`] role presently empty? (i.e. unprovisioned)
    pub fn is_empty(&self) -> bool {
        self.threshold == 0
    }

    /// Get the threshold of required keys
    #[allow(dead_code)]
    pub fn threshold(&self) -> usize {
        self.threshold
    }

    /// Get the public keys which are members of the root role
    #[allow(dead_code)]
    pub fn public_keys(&self) -> &[PublicKey] {
        self.public_keys.as_ref()
    }

    /// Get a UUID which represents this root configuration
    pub fn uuid(&self) -> Uuid {
        // TODO(tarcieri): stub!
        let mut uuid = Uuid::new();
        uuid.push_str("00000000-0000-0000-0000-000000000000")
            .unwrap();
        uuid
    }
}
