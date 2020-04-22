//! Armistice request messages

use crate::provision;
use veriform::{
    decoder::{Decodable, Decoder},
    field, Encoder, Error, Message,
};

/// Armistice request messages
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Request {
    /// Perform initial device provisioning
    // #[field(message, critical = true, tag = 0)]
    Provision(provision::Request),
}

// TODO(tarcieri): custom derive support for `veriform::Message`
impl Request {
    /// Get a provisioning request, if this is one
    pub fn provision(&self) -> Option<&provision::Request> {
        match self {
            Request::Provision(provision) => Some(provision),
        }
    }
}

// TODO(tarcieri): custom derive support for `veriform::Message`
impl From<provision::Request> for Request {
    fn from(request: provision::Request) -> Request {
        Request::Provision(request)
    }
}

// TODO(tarcieri): custom derive support for `veriform::Message`
impl Message for Request {
    fn decode(decoder: &mut Decoder, mut input: &[u8]) -> Result<Self, Error> {
        let header = decoder.peek().decode_header(&mut input)?;
        let message = decoder.peek().decode_message(&mut input)?;

        // TODO(tarcieri): higher-level abstraction for parsing enums
        decoder.push()?;

        let request = match header.tag {
            0 => Request::Provision(provision::Request::decode(decoder, &message)?),
            tag => {
                return Err(Error::FieldHeader {
                    tag: Some(tag),
                    wire_type: None,
                })
            }
        };

        if input.is_empty() {
            decoder.pop();
            Ok(request)
        } else {
            Err(Error::TrailingData)
        }
    }

    fn encode<'a>(&self, buffer: &'a mut [u8]) -> Result<&'a [u8], Error> {
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

#[cfg(test)]
pub(crate) mod tests {
    use super::Request;
    use crate::provision;
    use heapless::{consts::U128, Vec};
    use veriform::{Decoder, Message};

    /// Create an example `Request`
    pub(crate) fn example_message() -> Request {
        Request::Provision(provision::tests::example_request())
    }

    #[test]
    fn encoding_round_trip() {
        let request = example_message();

        let mut buffer: Vec<u8, U128> = Vec::new();
        buffer.extend_from_slice(&[0u8; 128]).unwrap();
        request.encode(&mut buffer).unwrap();
        buffer.truncate(request.encoded_len());

        let mut decoder = Decoder::new();
        assert_eq!(request, Request::decode(&mut decoder, &buffer).unwrap());
    }
}
