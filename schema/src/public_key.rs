//! Armistice public keys

use core::convert::TryInto;
use veriform::{
    decoder::{Decodable, Decoder},
    field::{self, WireType},
    message::{Element, Message},
    Encoder, Error,
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
    fn decode(decoder: &mut Decoder, mut input: &[u8]) -> Result<Self, Error> {
        let header = decoder.peek().decode_header(&mut input)?;
        let bytes = decoder.peek().decode_bytes(&mut input)?;

        let public_key = match header.tag {
            0 => bytes
                .try_into()
                .map(PublicKey::Ed25519)
                .map_err(|_| Error::Decode {
                    element: Element::Value,
                    wire_type: WireType::Bytes,
                })?,
            tag => {
                return Err(Error::FieldHeader {
                    tag: Some(tag),
                    wire_type: None,
                })
            }
        };

        if !input.is_empty() {
            return Err(Error::Decode {
                element: Element::Tag,
                wire_type: WireType::Message,
            });
        }

        Ok(public_key)
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
    use veriform::{Decoder, Message};

    #[test]
    fn encoding_round_trip_new() {
        let public_key = PublicKey::Ed25519([
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ]);

        let mut buffer: Vec<u8, U64> = Vec::new();
        buffer.extend_from_slice(&[0u8; 64]).unwrap();
        public_key.encode(&mut buffer).unwrap();
        buffer.truncate(public_key.encoded_len());

        let mut decoder = Decoder::new();
        assert_eq!(public_key, Message::decode(&mut decoder, &buffer).unwrap());
    }
}
