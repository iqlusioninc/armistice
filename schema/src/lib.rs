//! Armistice message schema definitions

#![no_std]
#![doc(html_root_url = "https://docs.rs/armistice_schema/0.0.0")]
#![forbid(unsafe_code)]
#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

pub mod provision;
pub mod public_key;
pub mod request;
pub mod response;

pub use self::{public_key::PublicKey, request::Request, response::Response};
