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

#[test]
#[ignore]
fn echo() {
    let results = armistice::usbarmory::run(&["testing", "123"]).unwrap();
    assert_eq!(results.len(), 2);
    assert_eq!(results[0], "gnitset");
    assert_eq!(results[1], "321");
}
