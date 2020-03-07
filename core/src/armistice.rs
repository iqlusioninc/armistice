//! Armistice core state

use crate::{
    crypto::{PublicKey, RootKey},
    error::Error,
    root,
    schema::{self, Request, Response},
};
use block_cipher_trait::{
    generic_array::{typenum::U16, ArrayLength, GenericArray},
    BlockCipher,
};

/// Armistice Core State
pub struct Armistice<B>
where
    B: BlockCipher<BlockSize = U16>,
    B::ParBlocks: ArrayLength<GenericArray<u8, B::BlockSize>>,
{
    /// Root configuration
    root_config: root::Config,

    /// Root symmetric key
    root_key: RootKey<B>,
}

impl<B> Armistice<B>
where
    B: BlockCipher<BlockSize = U16>,
    B::ParBlocks: ArrayLength<GenericArray<u8, B::BlockSize>>,
{
    /// Create new [`Armistice`] core state
    pub fn new(root_key: B) -> Self {
        Self {
            root_config: root::Config::default(),
            root_key: root_key.into(),
        }
    }

    /// Get the [`root::Config`]
    pub fn root_config(&self) -> &root::Config {
        &self.root_config
    }

    /// Get the [`RootKey`]
    pub fn root_key(&self) -> &RootKey<B> {
        &self.root_key
    }

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

        self.root_config = root::Config::new(threshold, keys)?;

        Ok(schema::provision::Response {
            uuid: self.root_config.uuid(),
        })
    }

    /// Are we already provisioned?
    pub fn is_provisioned(&self) -> bool {
        !self.root_config.is_empty()
    }
}
