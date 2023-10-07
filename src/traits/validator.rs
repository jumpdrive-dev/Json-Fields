use crate::errors::validation_error::ValidationError;
use serde_json::Value;

/// The primary trait that is used for validating a [Value]. This trait can be used to implement
/// custom validation logic for validating types that are not supported directly by the library.
///
/// ```rust
/// use std::str::FromStr;
/// use serde_json::{json, Value};
/// use thiserror::Error;
/// use uuid::Uuid;
/// use json_fields::errors::validation_error::ValidationError;
/// use json_fields::{Deserialize, Serialize, Validator, validator_impl};
///
/// #[derive(Serialize, Deserialize)]
/// struct UuidValidator;
///
/// #[derive(Debug, Error)]
/// enum UuidValidationError {
///     #[error("The provided UUID value is not a string")]
///     NotAString,
///
///     #[error("The provided UUID value is not correct")]
///     InvalidUuid,
/// }
///
/// #[validator_impl]
/// impl Validator for UuidValidator {
///     fn validate(&self, value: &Value) -> Result<(), ValidationError> {
///         let Value::String(string) = value else {
///             return Err(ValidationError::new_custom(UuidValidationError::NotAString));
///         };
///
///         Uuid::from_str(string)
///             .map_err(|_| ValidationError::new_custom(UuidValidationError::InvalidUuid))?;
///
///         Ok(())
///     }
/// }
///
/// let incorrect_uuid = json!("Hello world");
/// let correct_uuid = json!("550e8400-e29b-41d4-a716-446655440000");
///
/// let validator = UuidValidator;
///
/// assert!(validator.validate(&incorrect_uuid).is_err());
/// assert!(validator.validate(&correct_uuid).is_ok());
/// ```
///
/// The library re-exports [serde::Serialize] and [serde::Deserialize], but you can use those
/// directly from serde if that is desired. The same is true for [validator_impl]. It is a re-export
/// for [typetag::serde] and you can use that instead if you want.
#[typetag::serde(tag = "custom")]
pub trait Validator {
    /// Validates the provided value and check if it is valid. Return `Ok(())` to indicate that the
    /// value is correct and return a [ValidationError] if it's not correct with information
    /// as to why it did pass.
    fn validate(&self, value: &Value) -> Result<(), ValidationError>;
}
