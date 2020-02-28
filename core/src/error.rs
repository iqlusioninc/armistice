//! Error type

use displaydoc::Display;

/// Types of errors
#[derive(Copy, Clone, Debug, Display, Eq, PartialEq)]
pub enum Error {
    /// Crypto error
    Crypto,

    /// Provisioning error
    Provision,

    /// Threshold invalid
    Threshold,
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
