//! A simple Validator

use super::{
    extensions::simple_extension_uri::{SimpleExtensionURI, SimpleExtensionURIError},
    Context,
};
use std::collections::hash_map::Entry;
use thiserror::Error;
use url::Url;

/// A simple validator
pub struct Validator {
    simple_extensions: std::collections::HashMap<u32, Url>,
}

/// A validator error
#[derive(Debug, Error)]
pub enum ValidatorError {}

impl Context for Validator {
    fn register_simple_extension_uri(
        &mut self,
        simple_extension_uri: &SimpleExtensionURI,
    ) -> Result<(), SimpleExtensionURIError> {
        match self.simple_extensions.entry(simple_extension_uri.anchor()) {
            Entry::Occupied(other) => Err(SimpleExtensionURIError::DuplicateAnchor {
                added: Box::new(simple_extension_uri.clone()),
                defined: other.get().clone(),
            }),
            Entry::Vacant(entry) => {
                entry.insert(simple_extension_uri.uri().clone());
                Ok(())
            }
        }
    }
}
