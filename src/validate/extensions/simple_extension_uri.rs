//! Validation of [proto::extensions::SimpleExtensionUri]

use crate::{
    proto,
    validate::{Context, Validate},
};
use thiserror::Error;
use url::Url;

/// A validated [proto::extensions::SimpleExtensionUri]
#[derive(Clone, Debug, PartialEq)]
pub struct SimpleExtensionURI {
    uri: Url,
    anchor: u32,
}

impl SimpleExtensionURI {
    /// Returns the uri of this simple extension
    ///
    /// See [proto::extensions::SimpleExtensionUri::uri].
    pub fn uri(&self) -> &Url {
        &self.uri
    }

    /// Returns the anchor value of this simple extension
    ///
    /// See [proto::extensions::SimpleExtensionUri::extension_uri_anchor].
    pub fn anchor(&self) -> u32 {
        self.anchor
    }
}

/// Validation errors for [proto::extensions::SimpleExtensionUri]
#[derive(Debug, Error, PartialEq)]
pub enum SimpleExtensionURIError {
    /// The URI must be valid
    #[error("invalid URI `{0}`: {1}")]
    InvalidURI(String, #[source] url::ParseError),

    /// The anchor must be unique
    #[error("duplicate anchor `{}` (with URI `{}`) already used for {}", .added.anchor(), .added.uri(), .defined)]
    DuplicateAnchor {
        /// The simple extension URI that was added later
        added: Box<SimpleExtensionURI>,
        /// The simple extension URI that was already defined with this anchor
        defined: Url,
    },

    /// Depending on the validation context, a simple extension URI might be unsupported
    #[error("unsupported simple extension URI: {reason} (for URI `{}` with anchor `{}`)", .simple_extension_uri.uri(), .simple_extension_uri.anchor())]
    Unsupported {
        /// The unsupported simple extension URI
        simple_extension_uri: Box<SimpleExtensionURI>,
        /// The reason why this URI it unsupported
        reason: String,
    },
}

impl From<SimpleExtensionURI> for proto::extensions::SimpleExtensionUri {
    fn from(simple_extension_uri: SimpleExtensionURI) -> Self {
        let SimpleExtensionURI { uri, anchor } = simple_extension_uri;
        proto::extensions::SimpleExtensionUri {
            uri: uri.to_string(),
            extension_uri_anchor: anchor,
        }
    }
}

impl<C: Context> Validate<C> for proto::extensions::SimpleExtensionUri {
    type Validated = SimpleExtensionURI;
    type Error = SimpleExtensionURIError;

    fn validate(self, ctx: &mut C) -> Result<Self::Validated, Self::Error> {
        let proto::extensions::SimpleExtensionUri {
            extension_uri_anchor: anchor,
            uri,
        } = self;

        // Make sure the URI is valid
        let uri = Url::parse(&uri).map_err(move |e| SimpleExtensionURIError::InvalidURI(uri, e))?;

        let simple_extension_uri = SimpleExtensionURI { uri, anchor };

        // Make sure the URI is supported and the anchor is unique
        ctx.register_simple_extension_uri(&simple_extension_uri)?;

        Ok(simple_extension_uri)
    }
}
