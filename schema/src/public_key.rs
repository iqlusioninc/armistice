//! Armistice public keys

use core::convert::TryInto;
use veriform::{field, Decoder, Encoder, Error, Message};

/// Public keys
pub enum PublicKey {
    /// Ed25519 keys
    // #[field(bytes, tag = 0, size = 32)]
    Ed25519([u8; 32]),
}

// TODO(tarcieri): custom derive support for `veriform::Message`
impl Message for PublicKey {
    fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        let mut bytes = bytes.as_ref();
        let mut decoder = Decoder::new();

        let header = decoder.decode_header(&mut bytes)?;

        let result = match header.tag {
            0 => decoder.decode_bytes(&mut bytes).and_then(|slice| {
                slice
                    .try_into()
                    .map(PublicKey::Ed25519)
                    .map_err(|_| Error::Decode)
            })?,
            _ => return Err(Error::Decode),
        };

        if !bytes.is_empty() {
            return Err(Error::Decode);
        }

        Ok(result)
    }

    fn encode<'a>(&self, buffer: &'a mut [u8]) -> Result<&'a mut [u8], Error> {
        let mut encoder = Encoder::new(buffer);

        match self {
            PublicKey::Ed25519(bytes) => encoder.bytes(0, true, bytes)?,
        }

        Ok(encoder.finish())
    }

    fn encoded_len(&self) -> usize {
        match self {
            PublicKey::Ed25519(bytes) => field::length::bytes(0, true, bytes),
        }
    }
}
