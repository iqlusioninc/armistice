//! Armistice client integration tests
//!
//! Tests which require a USB armory MkII device running Armistice Core in
//! order to pass are tagged with `#[ignore]` and must be run with:
//!
//! ```text
//! $ cargo test -- --ignored
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

use armistice::{
    schema::{provision, public_key::PublicKey},
    Armistice,
};

#[test]
#[ignore]
fn perform_provisioning() {
    let mut armistice = Armistice::new().unwrap();

    let mut root_keys = provision::RootKeys::new();
    root_keys.push(PublicKey::Ed25519([0u8; 32])).unwrap();

    let digest = [
        62, 123, 47, 254, 135, 49, 169, 234, 97, 122, 186, 151, 131, 201, 204, 233, 10, 95, 192,
        185, 47, 98, 107, 121, 143, 244, 221, 127, 183, 159, 183, 187,
    ];

    let request = provision::Request {
        root_key_threshold: 1,
        root_keys,
        digest,
    };
    let response = armistice.send_request(request).unwrap();
    dbg!(&response);
}
