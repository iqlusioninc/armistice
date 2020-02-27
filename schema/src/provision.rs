//! Armistice device provisioning messages

use crate::public_key::PublicKey;
use heapless::{consts::U8, Vec};
use veriform::{
    decoder,
    field::{self, WireType},
    Decodable, Decoder, Encoder, Error, Message,
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
            let root_key_bytes = root_keys_decoder.decode_message(&mut root_keys_bytes)?;
            let root_key = PublicKey::decode(root_key_bytes)?;
            root_keys.push(root_key).map_err(|_| Error::Decode {
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

        let body_len: usize = self.root_keys.iter().map(|msg| msg.encoded_len()).sum();

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
