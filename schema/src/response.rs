//! Armistice response messages

use crate::provision;
use veriform::{
    decoder::{Decodable, Decoder},
    digest::Digest,
    field, Encoder, Error, Message,
};

/// Armistice response messages
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Response {
    /// Perform initial device provisioning
    // #[field(message, critical = true, tag = 0)]
    Provision(provision::Response),
}

// TODO(tarcieri): custom derive support for `veriform::Message`
impl Response {
    /// Get a provisioning response, if this is one
    pub fn provision(&self) -> Option<&provision::Response> {
        match self {
            Response::Provision(provision) => Some(provision),
        }
    }
}

impl From<provision::Response> for Response {
    fn from(response: provision::Response) -> Response {
        Response::Provision(response)
    }
}

// TODO(tarcieri): custom derive support for `veriform::Message`
impl Message for Response {
    fn decode<D>(decoder: &mut Decoder<D>, mut input: &[u8]) -> Result<Self, Error>
    where
        D: Digest,
    {
        let header = decoder.peek().decode_header(&mut input)?;
        let message = decoder.peek().decode_message(&mut input)?;

        // TODO(tarcieri): higher-level abstraction for parsing enums
        decoder.push()?;

        let response = match header.tag {
            0 => Response::Provision(provision::Response::decode(decoder, &message)?),
            tag => {
                return Err(Error::FieldHeader {
                    tag: Some(tag),
                    wire_type: None,
                })
            }
        };

        if input.is_empty() {
            decoder.pop();
            Ok(response)
        } else {
            Err(Error::TrailingData)
        }
    }

    fn encode<'a>(&self, buffer: &'a mut [u8]) -> Result<&'a [u8], Error> {
        let mut encoder = Encoder::new(buffer);

        match self {
            Response::Provision(msg) => encoder.message(0, true, msg)?,
        }

        Ok(encoder.finish())
    }

    fn encoded_len(&self) -> usize {
        match self {
            Response::Provision(msg) => field::length::message(0, msg),
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::Response;
    use crate::provision;
    use heapless::{consts::U128, Vec};
    use veriform::{Decoder, Message};

    /// Create an example `Response`
    pub(crate) fn example_message() -> Response {
        Response::Provision(provision::tests::example_response())
    }

    #[test]
    fn encoding_round_trip() {
        let response = example_message();

        let mut buffer: Vec<u8, U128> = Vec::new();
        buffer.extend_from_slice(&[0u8; 128]).unwrap();
        response.encode(&mut buffer).unwrap();
        buffer.truncate(response.encoded_len());

        let mut decoder = Decoder::new();
        assert_eq!(response, Response::decode(&mut decoder, &buffer).unwrap());
    }
}
