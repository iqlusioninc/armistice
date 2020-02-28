//! Armistice request messages

use crate::provision;
use veriform::{
    field,
    field::{Header, WireType},
    Decodable, Decoder, Encoder, Error, Message,
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
    fn decode(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
        let mut bytes = bytes.as_ref();
        let mut decoder = Decoder::new();
        let request = match decoder.decode_header(&mut bytes)? {
            Header {
                tag: 0,
                critical: true,
                wire_type: WireType::Message,
            } => Request::Provision(provision::Request::decode(
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
            Ok(request)
        } else {
            Err(Error::TrailingData)
        }
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

#[cfg(test)]
pub(crate) mod tests {
    use super::Request;
    use crate::provision;
    use heapless::{consts::U128, Vec};
    use veriform::Message;

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

        assert_eq!(request, Request::decode(&buffer).unwrap());
    }
}
