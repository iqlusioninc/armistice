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
    schema::{provision, PublicKey, Timestamp},
    Armistice,
};

#[test]
#[ignore]
fn perform_provisioning() {
    let mut armistice = Armistice::new().unwrap();

    let mut root_keys = provision::RootKeys::new();
    root_keys.push(PublicKey::Ed25519([0u8; 32])).unwrap();

    // TAI64N for 2020-05-21
    let timestamp =
        Timestamp::from_slice(&[64, 0, 0, 0, 94, 198, 207, 194, 32, 254, 206, 208]).unwrap();

    let request = provision::Request {
        root_key_threshold: 1,
        root_keys,
        timestamp,
        digest: None,
    };

    let response = armistice.send_request(request).unwrap();
    dbg!(&response);
}
