//! Armistice client

use crate::error::Error;
use armistice_schema::{veriform::Decoder, Message, Request, Response};

#[cfg(feature = "usbarmory")]
use crate::usbarmory;

/// Armistice client
// TODO(tarcieri): support for other connection methods besides USB
pub struct Armistice {
    /// USB armory connection
    usb: usbarmory::BulkPair,
}

impl Armistice {
    /// Open a connection to Armistice
    // TODO(tarcieri): credentials, non-USB connection support
    pub fn new() -> Result<Self, Error> {
        let usb = usbarmory::BulkPair::open(consts::VID, consts::PID)?;
        Ok(Armistice { usb })
    }

    /// Send a request to Armistice, parsing the response
    pub fn send_request(&mut self, request: impl Into<Request>) -> Result<Response, Error> {
        self.usb.write(&request.into().encode_vec()?)?;

        let mut buf = vec![0; self.usb.in_max_packet_size().into()];
        let response = self.usb.read(&mut buf)?;

        let mut decoder = Decoder::new();
        Ok(Response::decode(&mut decoder, response)?)
    }
}
