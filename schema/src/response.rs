//! Armistice response messages

use crate::provision;
use veriform::{
    field,
    field::{Header, WireType},
    Decodable, Decoder, Encoder, Error, Message,
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
    /// Get a provisioning request, if this is one
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
    fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        let mut bytes = bytes.as_ref();
        let mut decoder = Decoder::new();
        let response = match decoder.decode_header(&mut bytes)? {
            Header {
                tag: 0,
                critical: true,
                wire_type: WireType::Message,
            } => Response::Provision(provision::Response::decode(
                decoder.decode_message(&mut bytes)?,
            )?),
            Header { tag, wire_type, .. } => {
                return Err(Error::FieldHeader {
                    tag: Some(tag),
                    wire_type: Some(wire_type),
                })
            }
        };

        if bytes.is_empty() {
            Ok(response)
        } else {
            Err(Error::TrailingData)
        }
    }

    fn encode<'a>(&self, buffer: &'a mut [u8]) -> Result<&'a mut [u8], Error> {
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
    use veriform::Message;

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

        assert_eq!(response, Response::decode(&buffer).unwrap());
    }
}
