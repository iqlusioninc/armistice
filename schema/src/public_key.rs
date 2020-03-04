//! Armistice public keys

use core::convert::TryInto;
use veriform::{
    field::{self, WireType},
    message::Element,
    Decodable, Decoder, Encoder, Error, Message,
};

/// Public keys
#[derive(Clone, Debug, Eq, PartialEq)]
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
                    .map_err(|_| Error::Decode {
                        element: Element::Value,
                        wire_type: WireType::Bytes,
                    })
            })?,
            _ => {
                return Err(Error::Decode {
                    element: Element::Tag,
                    wire_type: WireType::Message,
                })
            }
        };

        if !bytes.is_empty() {
            return Err(Error::Decode {
                element: Element::Tag,
                wire_type: WireType::Message,
            });
        }

        Ok(result)
    }

    fn encode<'a>(&self, buffer: &'a mut [u8]) -> Result<&'a [u8], Error> {
        let mut encoder = Encoder::new(buffer);

        match self {
            PublicKey::Ed25519(bytes) => encoder.bytes(0, true, bytes)?,
        }

        Ok(encoder.finish())
    }

    fn encoded_len(&self) -> usize {
        match self {
            PublicKey::Ed25519(bytes) => field::length::bytes(0, bytes),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PublicKey;
    use heapless::{consts::U64, Vec};
    use veriform::Message;

    #[test]
    fn encoding_round_trip() {
        let public_key = PublicKey::Ed25519([
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ]);

        let mut buffer: Vec<u8, U64> = Vec::new();
        buffer.extend_from_slice(&[0u8; 64]).unwrap();
        public_key.encode(&mut buffer).unwrap();
        buffer.truncate(public_key.encoded_len());

        assert_eq!(public_key, PublicKey::decode(&buffer).unwrap());
    }
}
