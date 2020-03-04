//! Armistice core: cryptographic key storage and access control functionality
//! which runs as device firmware on the USB armory MkII.

#![no_std]
#![doc(html_root_url = "https://docs.rs/armistice_core/0.0.0")]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

#[cfg(feature = "std")]
extern crate std;

mod armistice;
pub mod crypto;
mod error;
mod root;

pub use armistice_schema as schema;
pub use heapless::{self, String, Vec};

pub use armistice::Armistice;
pub use error::Error;
