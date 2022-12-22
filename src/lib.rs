// SPDX-License-Identifier: Apache-2.0

//! [Substrait]: Cross-Language Serialization for Relational Algebra
//!
//! # Serialization and deserialization
//!
//! This crate provides generated types to serialize and deserialize Substrait
//! data.
//!
//! ## Protobuf
//!
//! Protobuf serialization and deserialization are provided via [prost] in the
//! [proto] module.
//!
//! ### Example
//!
//! #### Serialize and deserialize a plan
//! ```rust
//! # fn main() -> Result<(), prost::DecodeError> {
//! use prost::Message;
//! use substrait::proto::Plan;
//!
//! let plan = Plan::default();
//!
//! // Serialize the plan
//! let encoded = plan.encode_to_vec();
//!
//! // Deserialize the buffer to a Plan
//! let decoded = Plan::decode(encoded.as_slice())?;
//!
//! assert_eq!(plan, decoded);
//! # Ok(()) }
//! ```
//!
//! ### Serde support
//!
//! There are two (non-default) features available that provide derived
//! [Deserialize](serde::Deserialize) and [Serialize](serde::Serialize)
//! implementations for the generated types.
//!
//! Note that these features are mutually exclusive. The main difference between
//! those implementations are the field name case convention and field value
//! encoding. The examples below show how the `minor_number` field name in
//! [Version](proto::Version) matches the Protobuf field name with the `serde`
//! feature whereas it expects a lower camel case `minorNumber` field name with
//! the `pbjson` feature enabled. Please refer to the [Protobuf JSON Mapping]
//! documentation for more details.
//!
//! #### `serde`
//!
//! This adds `#[serde(Deserialize, Serialize)]` to all generated Protobuf
//! types. In addition, to match Protobuf defaults for missing optional data,
//! this adds `[serde(default)]` to all messages.
//!
//! ##### Example
//! ###### Deserialize a plan version using the `serde` feature
//! ```rust
//! # fn main() -> Result<(), serde_json::Error> {
//! # #[cfg(feature="serde")] {
//! use substrait::proto::Version;
//!
//! let version_json = r#"{
//!   "minor_number": 21
//! }"#;
//!
//! let version = serde_json::from_str::<Version>(version_json)?;
//! assert_eq!(
//!   version,
//!   Version {
//!     minor_number: 21,
//!     ..Default::default()
//!   }
//! );
//! # } Ok(()) }
//! ```
//!
//! #### `pbjson`
//!
//! This generates serde implementation that match the [Protobuf JSON Mapping]
//! via [pbjson].
//!
//! ##### Example
//! ###### Deserialize a plan version using the `pbjson` feature
//! ```rust
//! # fn main() -> Result<(), serde_json::Error> {
//! # #[cfg(feature="pbjson")] {
//! use substrait::proto::Version;
//!
//! let version_json = r#"{
//!   "minorNumber": 21
//! }"#;
//!
//! let version = serde_json::from_str::<Version>(version_json)?;
//! assert_eq!(
//!   version,
//!   Version {
//!     minor_number: 21,
//!     ..Default::default()
//!   }
//! );
//! # } Ok(()) }
//! ```
//!
//! ## Text
//!
//! Substrait defines a YAML schema for extensions. Types with serialization and
//! deserialization support for these are provided via [typify] in the [text]
//! module.
//!
//! ### Example
//!
//! #### Read a simple extension
//! ```rust
//! # fn main() -> Result<(), serde_yaml::Error> {
//! use substrait::text::simple_extensions::SimpleExtensions;
//!
//! let simple_extension_yaml = r#"
//! %YAML 1.2
//! ---
//! scalar_functions:
//!   -
//!     name: "add"
//!     description: "Add two values."
//!     impls:
//!       - args:
//!          - name: x
//!            value: i8
//!          - name: y
//!            value: i8
//!         options:
//!           overflow:
//!             values: [ SILENT, SATURATE, ERROR ]
//!         return: i8
//! "#;
//!
//! let simple_extension = serde_yaml::from_str::<SimpleExtensions>(simple_extension_yaml)?;
//!
//! assert_eq!(simple_extension.scalar_functions.len(), 1);
//! assert_eq!(simple_extension.scalar_functions[0].name, "add");
//! # Ok(()) }
//! ```
//!
//! [pbjson]: https://docs.rs/pbjson
//! [Protobuf JSON Mapping]:
//!     https://developers.google.com/protocol-buffers/docs/proto3#json
//! [Substrait]: https://substrait.io
//! [typify]: https://docs.rs/typify

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/substrait-io/substrait/main/site/docs/img/logo.svg",
    html_favicon_url = "https://raw.githubusercontent.com/substrait-io/substrait/main/site/docs/img/logo.svg"
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![deny(missing_docs)]

#[allow(clippy::needless_borrow, clippy::large_enum_variant, missing_docs)]
pub mod proto;
#[allow(clippy::uninlined_format_args, missing_docs)]
pub mod text;

pub mod version;

// Optional modules
#[cfg(feature = "validate")]
pub mod validate;

// #[derive(Error, Debug)]
// pub enum ReadRelError {
//     #[error("base schema is required")]
//     BaseSchemaMissing,
//     #[error("read relation definition is required")]
//     DefinitionMissing,
//     #[error("best effort filter expression error")]
//     Filter(#[from] ExpressionError),
// }

// trait Relation {
//     fn common(&self) -> &proto::RelCommon;
// }
// trait InputRelation: Relation {}

// // pub struct CommonRelation<T> {
// //     common: Option<proto::RelCommon>,
// //     relation: T,
// // }

// pub struct ReadRelation {}

// #[derive(Error, Debug)]
// pub enum ExpressionError {}

// impl<C: Context> Validate<C> for proto::Expression {
//     type Validated = Self;
//     type Error = ExpressionError;

//     fn validate(self, context: &mut C) -> Result<Self::Validated, Self::Error> {
//         Ok(self)
//     }
// }

// /// Type wrapper that validates that this expression is a predicate i.e. its return type is bool.
// pub struct PredicateExpression(proto::Expression);

// impl<C: Context> Validate<C> for PredicateExpression {
//     type Validated = proto::Expression;
//     type Error = ExpressionError;

//     fn validate(self, context: &mut C) -> Result<Self::Validated, Self::Error> {
//         // it should be a validate expression
//         self.0.validate(context)
//         // and it should return a bool
//     }
// }

// impl<C: Context> Validate<C> for proto::ReadRel {
//     type Validated = ReadRelation;
//     type Error = ReadRelError;

//     fn validate(self, context: &mut C) -> Result<Self::Validated, Self::Error> {
//         let proto::ReadRel {
//             common,
//             base_schema,
//             filter,
//             best_effort_filter,
//             projection,
//             advanced_extension,
//             read_type,
//         } = self;

//         let read_type = read_type.ok_or(ReadRelError::DefinitionMissing)?;
//         let base_schema = base_schema.ok_or(ReadRelError::BaseSchemaMissing)?;

//         let filer = filter
//             .map(|expression| PredicateExpression(*expression).validate(context))
//             .transpose()?;

//         let projection = projection
//             .map(|projection| projection.validate(context))
//             .transpose()?;

//         Ok(ReadRelation {})
//     }
// }

// pub trait Validate<C: Context> {
//     type Validated;
//     type Error: std::error::Error;

//     fn validate(self, context: &mut C) -> Result<Self::Validated, Self::Error>;
// }

// pub mod explain;
// pub mod relation;
