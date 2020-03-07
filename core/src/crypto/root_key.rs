//! Root key

use aes_gcm_siv::AesGcmSiv;

/// Root AES-GCM-SIV key
pub type RootKey<B> = AesGcmSiv<B>;
