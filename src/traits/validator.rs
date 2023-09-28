use serde_json::Value;
use crate::errors::validation_error::ValidationError;

/// The primary trait that is used for validating a [Value]. This trait can be used to implement
/// custom validation logic for validating types that are not supported directly by the library.
///
/// ```rust
/// use std::str::FromStr;
/// use serde_json::{json, Value};
/// use thiserror::Error;
/// use uuid::Uuid;
/// use json_fields::errors::validation_error::ValidationError;
/// use json_fields::Validator;
///
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
/// impl Validator for UuidValidator {
///     fn validate(&self, value: &Value) -> Result<(), ValidationError> {
///         let Value::String(string) = value else {
///             return Err(UuidValidationError::NotAString.into());
///         };
///
///         Uuid::from_str(string)
///             .map_err(|_| UuidValidationError::InvalidUuid)?;
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
pub trait Validator {
    fn validate(&self, value: &Value) -> Result<(), ValidationError>;
}
