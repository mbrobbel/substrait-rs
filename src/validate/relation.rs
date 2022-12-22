//! Validation of [proto::Rel]
//!
//!

use super::{Context, Validate};
use crate::proto;
use thiserror::Error;

/// A relation
#[derive(Debug, PartialEq)]
pub struct Relation {}

/// Validation error for [proto::Rel]
#[derive(Debug, Error, PartialEq)]
pub enum RelationError {}

impl From<Relation> for proto::Rel {
    fn from(_value: Relation) -> Self {
        todo!()
    }
}

impl<C: Context> Validate<C> for proto::Rel {
    type Validated = Relation;
    type Error = RelationError;

    fn validate(self, _ctx: &mut C) -> Result<Self::Validated, Self::Error> {
        todo!()
    }
}
