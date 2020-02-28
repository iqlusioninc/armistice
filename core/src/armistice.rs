//! Armistice Core State

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
    pub fn request(&mut self, request: Request) -> Result<Response, Error> {
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
        if self.root.is_empty() {
            self.root = Root::new(threshold, keys)?;
            Ok(schema::provision::Response {
                uuid: self.root.uuid(),
            })
        } else {
            Err(Error::Provision)
        }
    }
}
