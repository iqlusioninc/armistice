//! Armistice client: host-side access to the Armistice core application

#![doc(html_root_url = "https://docs.rs/armistice/0.0.0")]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

pub mod armistice;
pub mod error;

#[cfg(feature = "usbarmory")]
pub mod usbarmory;

pub use crate::{armistice::Armistice, error::Error};
pub use armistice_schema as schema;
