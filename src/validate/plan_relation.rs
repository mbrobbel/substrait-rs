//! Validation of [proto::PlanRel]

use super::{
    relation::{Relation, RelationError},
    root_relation::{RootRelation, RootRelationError},
    Context, Validate,
};
use crate::proto;
use thiserror::Error;

/// A validated [proto::PlanRel]
#[derive(Debug, PartialEq)]
pub enum PlanRelation {
    /// A relation
    Relation(Relation),

    /// A root relation with names for the output fields
    RootRelation(RootRelation),
}

impl From<PlanRelation> for proto::PlanRel {
    fn from(plan_relation: PlanRelation) -> Self {
        proto::PlanRel {
            rel_type: Some(plan_relation.into()),
        }
    }
}

impl From<PlanRelation> for proto::plan_rel::RelType {
    fn from(plan_relation: PlanRelation) -> Self {
        match plan_relation {
            PlanRelation::Relation(relation) => proto::plan_rel::RelType::Rel(relation.into()),
            PlanRelation::RootRelation(root_relation) => {
                proto::plan_rel::RelType::Root(root_relation.into())
            }
        }
    }
}

#[derive(Debug, Error, PartialEq)]
/// Validation errors for [proto::PlanRel]
pub enum PlanRelationError {
    /// A plan relation must either be a relation or root relation
    #[error("a plan must have one or more relations")]
    Empty,

    /// An error with the relation
    #[error("the relation is invalid: {0}")]
    Relation(#[from] RelationError),

    /// The root relation is invalid
    #[error("invalid root relation: {0}")]
    RootRelation(#[from] RootRelationError),
}

impl<C: Context> Validate<C> for proto::PlanRel {
    type Validated = PlanRelation;
    type Error = PlanRelationError;

    fn validate(self, ctx: &mut C) -> Result<Self::Validated, Self::Error> {
        self.rel_type
            .map(|rel_type| rel_type.validate(ctx))
            .transpose()?
            .ok_or(PlanRelationError::Empty)
    }
}

impl<C: Context> Validate<C> for proto::plan_rel::RelType {
    type Validated = PlanRelation;
    type Error = PlanRelationError;

    fn validate(self, ctx: &mut C) -> Result<Self::Validated, Self::Error> {
        Ok(match self {
            proto::plan_rel::RelType::Rel(relation) => PlanRelation::Relation(
                relation
                    .validate(ctx)
                    .map_err(PlanRelationError::Relation)?,
            ),
            proto::plan_rel::RelType::Root(root_relation) => PlanRelation::RootRelation(
                root_relation
                    .validate(ctx)
                    .map_err(PlanRelationError::RootRelation)?,
            ),
        })
    }
}
