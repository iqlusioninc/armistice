//! Root key

pub use aes_gcm_siv::ctr::*;
use aes_gcm_siv::AesGcmSiv;

/// Root AES-GCM-SIV key
pub type RootKey<B, C> = AesGcmSiv<B, C>;
