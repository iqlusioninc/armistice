//! Armistice request messages

use crate::provision::ProvisionRequest;
use veriform::{field, Encoder, Error, Message};

/// Armistice request messages
pub enum Request {
    /// Perform initial device provisioning
    // #[field(message, tag = 0)]
    Provision(ProvisionRequest),
}

// TODO(tarcieri): custom derive support for `veriform::Message`
impl Message for Request {
    fn decode(_bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        unimplemented!();
    }

    fn encode<'a>(&self, buffer: &'a mut [u8]) -> Result<&'a mut [u8], Error> {
        let mut encoder = Encoder::new(buffer);

        match self {
            Request::Provision(msg) => encoder.message(0, true, msg)?,
        }

        Ok(encoder.finish())
    }

    fn encoded_len(&self) -> usize {
        match self {
            Request::Provision(msg) => field::length::message(0, msg),
        }
    }
}
