//! Armistice response messages

use crate::provision;
use veriform::Message;

/// Armistice response messages
#[derive(Message, Clone, Debug, Eq, PartialEq)]
pub enum Response {
    /// Perform initial device provisioning
    #[field(tag = 0, wire_type = "message")]
    Provision(provision::Response),
}

// TODO(tarcieri): add to custom derive support for `veriform::Message`
impl Response {
    /// Get a provisioning response, if this is one
    pub fn provision(&self) -> Option<&provision::Response> {
        match self {
            Response::Provision(provision) => Some(provision),
        }
    }
}

// TODO(tarcieri): add to custom derive support for `veriform::Message`
impl From<provision::Response> for Response {
    fn from(response: provision::Response) -> Response {
        Response::Provision(response)
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
