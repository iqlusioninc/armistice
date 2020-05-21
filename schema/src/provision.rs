//! Armistice device provisioning messages: performs initial device setup

use crate::{public_key::PublicKey, Timestamp, Uuid};
use heapless::{consts::U8, Vec};
use veriform::{Message, Sha256Digest};

/// Root keys collection
pub type RootKeys = Vec<PublicKey, U8>;

/// Request to provision a device
#[derive(Message, Clone, Debug, Eq, PartialEq)]
pub struct Request {
    /// Number of signatures required to perform root key operations
    #[field(tag = 0, wire_type = "uint64", critical = true, max = 8)]
    pub root_key_threshold: u64,

    /// Root keys used to manage domains
    #[field(tag = 1, wire_type = "sequence", critical = true, max = 8)]
    pub root_keys: RootKeys,

    /// Date/time when provisioning occurs (agreed upon by all root keys)
    #[field(tag = 2, wire_type = "message", critical = true)]
    pub timestamp: Timestamp,

    /// Digest of this message (to be signed by each of the root keys)
    #[digest(alg = "sha256")]
    pub digest: Option<Sha256Digest>,
}

/// Response to a device being provisioned
#[derive(Message, Clone, Debug, Eq, PartialEq)]
pub struct Response {
    /// UUID (deterministically) assigned at provisioning time
    #[field(tag = 0, wire_type = "message", critical = true)]
    pub uuid: Uuid,
}

#[cfg(test)]
pub(crate) mod tests {
    use super::{Request, Response};
    use crate::{PublicKey, Timestamp, Uuid};
    use heapless::{consts::U128, Vec};
    use veriform::{Decoder, Message};

    /// Get an example timestamp
    fn example_timestamp() -> Timestamp {
        // TAI64N for 2020-05-21
        Timestamp::from_slice(&[64, 0, 0, 0, 94, 198, 207, 194, 32, 254, 206, 208]).unwrap()
    }

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

        let expected_digest = [
            225, 37, 108, 176, 102, 183, 208, 121, 75, 83, 180, 236, 23, 212, 221, 75, 18, 204, 98,
            114, 146, 40, 65, 128, 2, 84, 116, 235, 219, 32, 123, 148,
        ];

        Request {
            root_key_threshold: 1,
            root_keys,
            timestamp: example_timestamp(),
            digest: Some(expected_digest),
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
