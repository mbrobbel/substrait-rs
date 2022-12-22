//! Validation of [proto::Plan]

use super::{
    extensions::simple_extension_uri::{SimpleExtensionURI, SimpleExtensionURIError},
    plan_relation::PlanRelationError,
    version::{Version, VersionError},
    Context, Validate,
};
use crate::{proto, version};
use thiserror::Error;

/// A validated [proto::Plan]
#[derive(Debug, PartialEq)]
pub struct Plan {
    version: Version,
    simple_extension_uris: Vec<SimpleExtensionURI>,
}

impl From<Plan> for proto::Plan {
    fn from(plan: Plan) -> Self {
        let Plan {
            version,
            simple_extension_uris,
        } = plan;
        let _ = proto::Plan {
            version: Some(version.into()),
            extension_uris: simple_extension_uris.into_iter().map(Into::into).collect(),
            ..Default::default()
        };
        todo!()
    }
}

/// Validation errors for [proto::Plan]
#[derive(Debug, Error, PartialEq)]
pub enum PlanError {
    /// The version of this plan is invalid
    #[error("invalid version")]
    Version(#[from] VersionError),

    /// The Substrait version of this plan is incompatible
    #[error("substrait version mismatch (plan version: `{0}`, supported: `{1}`")]
    SubstraitVersion(semver::Version, semver::VersionReq),

    /// There is an error with a simple extension
    #[error("failed to register simple extension: `{0}`")]
    SimpleExtension(#[from] SimpleExtensionURIError),

    /// There must be a least one relation
    #[error("plan has no relations")]
    MissingRelations,

    /// An issue with a plan relation
    #[error("plan relation error: {0}")]
    PlanRelation(#[from] PlanRelationError),
}

impl<C: Context> Validate<C> for proto::Plan {
    type Validated = Plan;
    type Error = PlanError;

    fn validate(self, ctx: &mut C) -> Result<Self::Validated, Self::Error> {
        let proto::Plan {
            version,
            extension_uris,
            extensions: _,
            relations,
            advanced_extensions: _,
            expected_type_urls: _,
        } = self;

        // A plan requires a version, and it must be valid
        let version = version
            .map(|version| version.validate(ctx))
            .transpose()?
            .ok_or(VersionError::Missing)?;

        // The version must be compatible
        if !version::semver_req().matches(&version.version) {
            return Err(PlanError::SubstraitVersion(
                version.version,
                version::semver_req(),
            ));
        }

        // Validate simple extension URIs
        let simple_extension_uris = extension_uris
            .into_iter()
            .map(|extension_uri| extension_uri.validate(ctx))
            .collect::<Result<_, _>>()?;

        // Validate simple extension definitions
        // extensions
        //     .into_iter()
        //     .try_for_each(|extension| extension.validate(context).map(|_| ()))?;

        // Validate plan relations
        let _plan_relations = relations
            .into_iter()
            .map(|plan_relation| plan_relation.validate(ctx))
            .collect::<Result<Vec<_>, _>>()?;

        // Make sure there is at least one root relation?

        Ok(Plan {
            version,
            simple_extension_uris,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Context;

    impl super::Context for Context {}

    #[test]
    fn version() {
        // Version could be missing
        let plan = proto::Plan {
            version: None,
            ..Default::default()
        };
        assert_eq!(
            plan.validate(&mut Context),
            Err(PlanError::Version(VersionError::Missing))
        );

        // Version could be invalid
        let plan = proto::Plan {
            version: Some(proto::Version::default()),
            ..Default::default()
        };
        assert_eq!(
            plan.validate(&mut Context),
            Err(PlanError::Version(VersionError::Missing))
        );

        // Version could be incompatible
        let plan = proto::Plan {
            version: Some(proto::Version {
                major_number: 0,
                minor_number: 0,
                patch_number: 42,
                ..Default::default()
            }),
            ..Default::default()
        };
        assert!(matches!(
            plan.validate(&mut Context),
            Err(PlanError::SubstraitVersion(_, _))
        ));
    }
}
