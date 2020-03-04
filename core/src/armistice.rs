//! Armistice core state

use crate::{
    crypto::PublicKey,
    error::Error,
    root::Root,
    schema::{self, Request, Response},
};

/// Armistice Core State
#[derive(Debug, Default)]
pub struct Armistice {
    /// Root configuration
    root: Root,
}

impl Armistice {
    /// Process the given [`Request`], returning a [`Response`] or an [`Error`]
    pub fn handle_request(&mut self, request: Request) -> Result<Response, Error> {
        match request {
            Request::Provision(provision) => self
                .provision(
                    provision.root_key_threshold as usize,
                    provision.root_keys.into_iter().map(Into::into),
                )
                .map(Into::into),
        }
    }

    /// Perform initial device provisioning
    pub fn provision(
        &mut self,
        threshold: usize,
        keys: impl IntoIterator<Item = PublicKey>,
    ) -> Result<schema::provision::Response, Error> {
        if self.is_provisioned() {
            return Err(Error::Provision);
        }

        self.root = Root::new(threshold, keys)?;

        Ok(schema::provision::Response {
            uuid: self.root.uuid(),
        })
    }

    /// Are we already provisioned?
    pub fn is_provisioned(&self) -> bool {
        !self.root.is_empty()
    }
}
