//! Armistice device provisioning messages: performs initial device setup

use crate::{public_key::PublicKey, Uuid};
use heapless::{consts::U8, Vec};
use veriform::{
    decoder::{sequence, Decode, DecodeSeq, Decoder},
    digest::Digest,
    error::{self, Error},
    field::{self, WireType},
    message::Element,
    vint64, Encoder, Message,
};

/// Root keys collection
pub type RootKeys = Vec<PublicKey, U8>;

/// Request to provision a device
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request {
    /// Number of signatures required to perform root key operations
    // #[field(uint64, tag = 0, critical = true, max = 8)]
    pub root_key_threshold: u64,

    /// Root keys used to manage domains
    // #[field(sequence(message), tag = 1, critical = true, max = 8)]
    pub root_keys: RootKeys,

    /// Digest of this message (to be signed by each of the root keys)
    // #[digest(sha256)]
    pub digest: [u8; 32],
}

/// Response to a device being provisioned
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Response {
    /// UUID (deterministically) assigned at provisioning time
    // #[field(message, tag = 0, critical = true)]
    pub uuid: Uuid,
}

// TODO(tarcieri): custom derive support for `veriform::Message`
impl Message for Request {
    fn decode<D>(decoder: &mut Decoder<D>, mut input: &[u8]) -> Result<Self, Error>
    where
        D: Digest,
    {
        let root_key_threshold = decoder.decode(0, &mut input)?;
        let root_keys_iter: sequence::Iter<'_, PublicKey, D> = decoder.decode_seq(1, &mut input)?;
        let mut root_keys = Vec::new();

        for root_key in root_keys_iter {
            root_keys.push(root_key?).map_err(|_| {
                Error::from(error::Kind::Decode {
                    element: Element::Value,
                    wire_type: WireType::Sequence,
                })
            })?;
        }

        let mut digest = [0u8; 32];
        decoder.fill_digest(&mut digest)?;

        Ok(Self {
            root_key_threshold,
            root_keys,
            digest,
        })
    }

    fn encode<'a>(&self, buffer: &'a mut [u8]) -> Result<&'a [u8], Error> {
        let mut encoder = Encoder::new(buffer);
        encoder.uint64(0, true, self.root_key_threshold)?;

        let body_len: usize = self
            .root_keys
            .iter()
            .map(|msg| {
                // compute length with additional length prefix
                let encoded_len = msg.encoded_len();
                vint64::encoded_len(encoded_len as u64)
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

// TODO(tarcieri): custom derive support for `veriform::Message`
impl Message for Response {
    fn decode<D>(decoder: &mut Decoder<D>, mut input: &[u8]) -> Result<Self, Error>
    where
        D: Digest,
    {
        decoder.decode(0, &mut input).map(|uuid| Self { uuid })
    }

    fn encode<'a>(&self, buffer: &'a mut [u8]) -> Result<&'a [u8], Error> {
        let mut encoder = Encoder::new(buffer);
        encoder.message(0, true, &self.uuid)?;
        Ok(encoder.finish())
    }

    fn encoded_len(&self) -> usize {
        field::length::message(0, &self.uuid)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::{Request, Response};
    use crate::public_key::PublicKey;
    use heapless::{consts::U128, Vec};
    use veriform::{builtins::Uuid, Decoder, Message};

    /// Create an example `provision::Request`
    pub(crate) fn example_request() -> Request {
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

        let digest = [
            62, 123, 47, 254, 135, 49, 169, 234, 97, 122, 186, 151, 131, 201, 204, 233, 10, 95,
            192, 185, 47, 98, 107, 121, 143, 244, 221, 127, 183, 159, 183, 187,
        ];

        Request {
            root_key_threshold: 1,
            root_keys,
            digest,
        }
    }

    /// Create an example `provision::Response`
    pub(crate) fn example_response() -> Response {
        let uuid = Uuid::parse_str("88888888-4444-4444-4444-121212121212").unwrap();
        Response { uuid }
    }

    #[test]
    fn request_round_trip() {
        let request = example_request();

        let mut buffer: Vec<u8, U128> = Vec::new();
        buffer.extend_from_slice(&[0u8; 128]).unwrap();
        request.encode(&mut buffer).unwrap();
        buffer.truncate(request.encoded_len());

        let mut decoder = Decoder::new();
        assert_eq!(request, Request::decode(&mut decoder, &buffer).unwrap());
    }

    #[test]
    fn response_round_trip() {
        let response = example_response();

        let mut buffer: Vec<u8, U128> = Vec::new();
        buffer.extend_from_slice(&[0u8; 128]).unwrap();
        response.encode(&mut buffer).unwrap();
        buffer.truncate(response.encoded_len());

        let mut decoder = Decoder::new();
        assert_eq!(response, Response::decode(&mut decoder, &buffer).unwrap());
    }
}
