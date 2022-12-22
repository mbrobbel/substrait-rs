//! Validation of [proto::RelRoot]
//!
//!

use crate::proto;
use thiserror::Error;

use super::{
    relation::{Relation, RelationError},
    Context, Validate,
};

/// A root relation
#[derive(Debug, PartialEq)]
pub struct RootRelation {
    /// The relation
    relation: Relation,
    /// The names for the output fields (depth-first)
    schema: Vec<String>, // todo
}

/// Validation errors for [proto::RelRoot]
#[derive(Debug, Error, PartialEq)]
pub enum RootRelationError {
    /// Invalid relation
    #[error("invalid relation: {0}")]
    Relation(#[from] RelationError),
}

impl From<RootRelation> for proto::RelRoot {
    fn from(_value: RootRelation) -> Self {
        todo!()
    }
}

impl<C: Context> Validate<C> for proto::RelRoot {
    type Validated = RootRelation;
    type Error = RootRelationError;

    fn validate(self, _ctx: &mut C) -> Result<Self::Validated, Self::Error> {
        todo!()
    }
}
