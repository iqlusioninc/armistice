//! Armistice public keys

use veriform::Message;

/// Public keys
#[derive(Message, Clone, Debug, Eq, PartialEq)]
pub enum PublicKey {
    /// Ed25519 keys
    #[field(tag = 0, wire_type = "bytes", size = 32)]
    Ed25519([u8; 32]),
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
