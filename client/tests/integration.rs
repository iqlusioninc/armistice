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

    let request = provision::Request {
        root_key_threshold: 1,
        root_keys,
    };
    let response = armistice.send_request(request).unwrap();
    dbg!(&response);
}
