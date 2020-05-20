//! Armistice request messages

use crate::provision;
use veriform::Message;

/// Armistice request messages
#[derive(Message, Clone, Debug, Eq, PartialEq)]
pub enum Request {
    /// Perform initial device provisioning
    #[field(tag = 0, wire_type = "message")]
    Provision(provision::Request),
}

// TODO(tarcieri): add to custom derive support for `veriform::Message`
impl Request {
    /// Get a provisioning request, if this is one
    pub fn provision(&self) -> Option<&provision::Request> {
        match self {
            Request::Provision(provision) => Some(provision),
        }
    }
}

// TODO(tarcieri): add to custom derive support for `veriform::Message`
impl From<provision::Request> for Request {
    fn from(request: provision::Request) -> Self {
        Request::Provision(request)
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
