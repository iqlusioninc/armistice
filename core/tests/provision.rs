//! Provisioning integration test

use aes::{block_cipher_trait::BlockCipher, Aes128};
use armistice_core::Vec;
use armistice_schema::{provision, public_key::PublicKey, Uuid};

type Armistice = armistice_core::Armistice<Aes128>;

#[test]
fn provisioning_happy_path() {
    let root_encryption_key = Aes128::new(
        &[
            0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xad,
            0xbe, 0xef,
        ]
        .into(),
    );

    let mut armistice = Armistice::new(root_encryption_key);
    let mut root_keys = Vec::new();

    root_keys
        .push(PublicKey::Ed25519([
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31,
        ]))
        .unwrap();
    root_keys
        .push(PublicKey::Ed25519([
            31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10,
            9, 8, 7, 6, 5, 4, 3, 2, 1, 0,
        ]))
        .unwrap();

    let request = provision::Request {
        root_key_threshold: 1,
        root_keys,
        digest: None,
    };

    let response = armistice.handle_request(request.into()).unwrap();

    // TODO(tarcieri): stub!
    assert_eq!(
        response.provision().unwrap().uuid,
        Uuid::parse_str("00000000-0000-0000-0000-000000000000").unwrap()
    );
}
