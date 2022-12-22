//! Validation of Substrait data
//!
//! Some requirements of Substrait data can't be expressed via Protobuf
//! definition or schema files. This module provides validated new types for the
//! generated types, that when constructed are known to be valid. This allows
//! producers and consumers skip checks to make sure that they follow the strict
//! invariants of the specification. The types by construction should provide
//! those guarantees.

use self::extensions::simple_extension_uri::{SimpleExtensionURI, SimpleExtensionURIError};
use std::error::Error;

pub mod extensions;
pub mod plan;
pub mod plan_relation;
pub mod relation;
pub mod root_relation;
pub mod validator;
pub mod version;

/// A validation context
pub trait Context {
    // type Error: std::error::Error;

    /// Validate an item using this context.
    ///
    /// See [Validate::validate].
    fn validate<T: Validate<Self>>(&mut self, item: T) -> Result<T::Validated, T::Error>
    where
        Self: Sized,
    {
        item.validate(self)
    }

    /// Registers a [SimpleExtensionURI]. Should return an error for duplicate
    /// anchors ([SimpleExtensionURIError::DuplicateAnchor]) or when the URI is
    /// not supported ([SimpleExtensionURIError::Unsupported]).
    ///
    /// This function may try to eagerly resolve and validate the simple
    /// extension, but it can't report resolve errors until the extension is referenced,
    /// or when checking for unused extensions.
    fn register_simple_extension_uri(
        &mut self,
        simple_extension_uri: &SimpleExtensionURI,
    ) -> Result<(), SimpleExtensionURIError> {
        let _ = simple_extension_uri;
        unimplemented!("this context does not support simple extension URIs")
    }

    // fn unused_simple_extensions()
}

/// A validation trait
pub trait Validate<C: Context>: Sized {
    /// The validated type
    ///
    /// After validation this type must be able to convert back. Note that it is
    /// not required for the conversion to be lossless, as long as the semantics
    /// don't change.
    ///
    /// This bound also helps with tracking breaking Protobuf definition changes
    /// via compilation errors.
    type Validated: Into<Self>;

    /// The error type for this validation
    type Error: Error;

    /// Validate and return a validated type or error
    fn validate(self, ctx: &mut C) -> Result<Self::Validated, Self::Error>;
}
