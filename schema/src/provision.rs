//! Armistice device provisioning messages

use crate::public_key::PublicKey;
use heapless::{consts::U8, Vec};
use veriform::{
    decoder,
    field::{self, WireType},
    message::Element,
    vint64, Decodable, Decoder, Encoder, Error, Message,
};

/// Request to provision a device
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvisionRequest {
    /// Number of signatures required to perform root key operations
    // #[field(uint64, tag = 0, critical = true, max = 8)]
    pub root_key_threshold: u64,

    /// Root keys used to manage domains
    // #[field(sequence(message), tag = 1, critical = true, max = 8)]
    pub root_keys: Vec<PublicKey, U8>,
}

// TODO(tarcieri): custom derive support for `veriform::Message`
impl Message for ProvisionRequest {
    fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        let mut bytes = bytes.as_ref();
        let mut decoder = Decoder::new();

        decoder.decode_expected_header(&mut bytes, 0, WireType::UInt64)?;
        let root_key_threshold = decoder.decode_uint64(&mut bytes)?;

        decoder.decode_expected_header(&mut bytes, 1, WireType::Sequence)?;

        let mut root_keys_bytes = decoder.decode_sequence(WireType::Message, &mut bytes)?;
        let mut root_keys = Vec::new();
        let mut root_keys_decoder =
            decoder::sequence::Decoder::new(WireType::Message, root_keys_bytes.len());

        while !root_keys_bytes.is_empty() {
            let root_key =
                PublicKey::decode(root_keys_decoder.decode_message(&mut root_keys_bytes)?)?;

            root_keys.push(root_key).map_err(|_| Error::Decode {
                element: Element::Value,
                wire_type: WireType::Sequence,
            })?;
        }

        Ok(Self {
            root_key_threshold,
            root_keys,
        })
    }

    fn encode<'a>(&self, buffer: &'a mut [u8]) -> Result<&'a mut [u8], Error> {
        let mut encoder = Encoder::new(buffer);
        encoder.uint64(0, true, self.root_key_threshold)?;

        let body_len: usize = self
            .root_keys
            .iter()
            .map(|msg| {
                // compute length with additional length prefix
                let encoded_len = msg.encoded_len();
                vint64::encode(encoded_len as u64)
                    .len()
                    .checked_add(encoded_len)
                    .unwrap()
            })
            .sum();

        encoder.message_seq(
            1,
            true,
            body_len,
            self.root_keys.iter().map(|key| key as &dyn Message),
        )?;

        Ok(encoder.finish())
    }

    fn encoded_len(&self) -> usize {
        field::length::uint64(0, self.root_key_threshold)
            + field::length::message_seq(1, self.root_keys.iter().map(|key| key as &dyn Message))
    }
}

#[cfg(test)]
mod tests {
    use super::ProvisionRequest;
    use crate::public_key::PublicKey;
    use heapless::{consts::U128, Vec};
    use veriform::Message;

    #[test]
    fn serialization_round_trip() {
        let mut root_keys = Vec::new();
        root_keys
            .push(PublicKey::Ed25519([
                0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                23, 24, 25, 26, 27, 28, 29, 30, 31,
            ]))
            .unwrap();
        root_keys
            .push(PublicKey::Ed25519([
                31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11,
                10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,
            ]))
            .unwrap();

        let provision_request = ProvisionRequest {
            root_key_threshold: 1,
            root_keys,
        };

        let mut buffer: Vec<u8, U128> = Vec::new();
        buffer.extend_from_slice(&[0u8; 128]).unwrap();

        provision_request.encode(&mut buffer).unwrap();
        buffer.truncate(provision_request.encoded_len());

        let provision_request_decoded = ProvisionRequest::decode(&buffer).unwrap();
        assert_eq!(provision_request, provision_request_decoded);
    }
}
