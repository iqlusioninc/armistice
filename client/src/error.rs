//! Error types

use anomaly::BoxError;
use displaydoc::Display;
use std::{
    fmt::{self, Display},
    ops::Deref,
};

/// Kinds of errors
#[derive(Clone, Debug, Display, Eq, PartialEq)]
pub enum Kind {
    /// Encoding error
    Encoding,

    /// USB error
    Usb,
}

impl Kind {
    /// Add additional context to this error
    pub fn context(self, source: impl Into<BoxError>) -> Context {
        Context::new(self, Some(source.into()))
    }
}

impl std::error::Error for Kind {}

/// Error contexts
pub type Context = anomaly::Context<Kind>;

/// Error type
#[derive(Debug)]
pub struct Error(Box<Context>);

impl Deref for Error {
    type Target = Context;

    fn deref(&self) -> &Context {
        &self.0
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

impl From<Kind> for Error {
    fn from(kind: Kind) -> Self {
        Context::new(kind, None).into()
    }
}

impl From<Context> for Error {
    fn from(context: Context) -> Self {
        Error(Box::new(context))
    }
}

impl From<rusb::Error> for Error {
    fn from(err: rusb::Error) -> Error {
        Kind::Usb.context(err).into()
    }
}

impl From<veriform::Error> for Error {
    fn from(err: veriform::Error) -> Error {
        Kind::Encoding.context(err).into()
    }
}
