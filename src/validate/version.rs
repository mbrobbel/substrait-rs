//! Validation of [proto::Version]

use super::{Context, Validate};
use crate::proto;
use hex::FromHex;
use thiserror::Error;

/// A validated [proto::Version]
#[derive(Debug, PartialEq)]
pub struct Version {
    /// The semantic version
    pub version: semver::Version,
    /// The git hash if set as bytes
    pub git_hash: Option<[u8; 20]>,
    /// The producer string if set
    pub producer: Option<String>,
}

impl Version {
    /// Returns the version
    pub fn version(&self) -> &semver::Version {
        &self.version
    }

    /// Returns the git hash
    pub fn git_hash(&self) -> Option<&[u8; 20]> {
        self.git_hash.as_ref()
    }

    /// Returns the producer
    pub fn producer(&self) -> Option<&str> {
        self.producer.as_deref()
    }
}

impl From<Version> for proto::Version {
    fn from(version: Version) -> Self {
        let Version {
            version,
            git_hash,
            producer,
        } = version;
        proto::Version {
            major_number: version.major.try_into().unwrap(),
            minor_number: version.minor.try_into().unwrap(),
            patch_number: version.patch.try_into().unwrap(),
            git_hash: git_hash.map(hex::encode).unwrap_or_default(),
            producer: producer.unwrap_or_default(),
        }
    }
}

/// Validation errors for [proto::Version]
#[derive(Debug, Error, PartialEq)]
pub enum VersionError {
    /// An issues with the git hash
    #[error(
        "git hash must be a lowercase hex ASCII string, 40 characters in length: (git hash: {0})"
    )]
    GitHash(String),

    /// Version is missing
    #[error("version must be specified")]
    Missing,
}

impl<C: Context> Validate<C> for proto::Version {
    type Validated = Version;
    type Error = VersionError;

    fn validate(self, _ctx: &mut C) -> Result<Self::Validated, Self::Error> {
        let proto::Version {
            major_number,
            minor_number,
            patch_number,
            git_hash,
            producer,
        } = self;

        // All version numbers unset (u32::default()) is an error, because
        // version is required
        if major_number == u32::default()
            && minor_number == u32::default()
            && patch_number == u32::default()
        {
            return Err(VersionError::Missing);
        }

        // The git hash, when set, must be a lowercase hex ASCII string, 40 characters in length
        if !git_hash.is_empty()
            && (git_hash.len() != 40
                || !git_hash.chars().all(|x| matches!(x, '0'..='9' | 'a'..='f')))
        {
            return Err(VersionError::GitHash(git_hash));
        }

        Ok(Version {
            version: semver::Version::new(
                major_number.into(),
                minor_number.into(),
                patch_number.into(),
            ),
            git_hash: (!git_hash.is_empty()).then(|| <[u8; 20]>::from_hex(git_hash).unwrap()),
            producer: (!producer.is_empty()).then_some(producer),
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
        let version = proto::Version::default();
        assert_eq!(version.validate(&mut Context), Err(VersionError::Missing));

        let version = proto::Version {
            major_number: 1,
            ..Default::default()
        };
        assert!(version.validate(&mut Context).is_ok());
    }

    #[test]
    fn git_hash() {
        let base = proto::Version {
            major_number: 1,
            ..Default::default()
        };

        // Bad length
        let git_hash = String::from("short");
        let version = proto::Version {
            git_hash: git_hash.clone(),
            ..base.clone()
        };
        assert_eq!(
            version.validate(&mut Context),
            Err(VersionError::GitHash(git_hash))
        );

        // Not lowercase
        let git_hash = String::from("2FD4E1C67A2D28FCED849EE1BB76E7391B93EB12");
        let version = proto::Version {
            git_hash: git_hash.clone(),
            ..base.clone()
        };
        assert_eq!(
            version.validate(&mut Context),
            Err(VersionError::GitHash(git_hash))
        );

        // Not all hex digits
        let git_hash = String::from("2fd4e1c67a2d28fced849ee1bb76e7391b93eb1g");
        let version = proto::Version {
            git_hash: git_hash.clone(),
            ..base.clone()
        };
        assert_eq!(
            version.validate(&mut Context),
            Err(VersionError::GitHash(git_hash))
        );

        // Not all ascii
        let git_hash = String::from("2fd4e1c67a2d28fced849ee1bb76e7391b93eb1å");
        let version = proto::Version {
            git_hash: git_hash.clone(),
            ..base.clone()
        };
        assert_eq!(
            version.validate(&mut Context),
            Err(VersionError::GitHash(git_hash))
        );

        // Valid
        let git_hash = String::from("2fd4e1c67a2d28fced849ee1bb76e7391b93eb12");
        let version = proto::Version {
            git_hash,
            ..base.clone()
        };
        assert!(version.validate(&mut Context).is_ok());
    }

    #[test]
    fn producer() {
        // Empty producer maps to `None`
        let version = proto::Version {
            major_number: 1,
            producer: String::from(""),
            ..Default::default()
        };
        assert!(version.validate(&mut Context).unwrap().producer.is_none());
    }

    #[test]
    fn convert() {
        let version = proto::Version {
            major_number: 1,
            minor_number: 2,
            patch_number: 3,
            git_hash: String::from(""),
            producer: String::from("test"),
        };
        assert_eq!(
            proto::Version::from(version.clone().validate(&mut Context).unwrap()),
            version
        );
    }
}
