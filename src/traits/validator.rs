use serde_json::Value;
use crate::errors::validation_error::ValidationError;

/// The primary trait that is used for validating a [Value]. Can be used to implement custom
/// validators for external types, for example when needing to validate a UUID:
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
///
/// You can also then use the validator as a custom field when nesting it within other fields like
/// in an [ObjectField]:
///
/// ```rust
/// # use std::str::FromStr;
/// # use serde_json::{json, Value};
/// # use thiserror::Error;
/// # use uuid::Uuid;
/// # use json_fields::errors::validation_error::ValidationError;
/// # use json_fields::Validator;
/// #
/// # struct UuidValidator;
/// #
/// # #[derive(Debug, Error)]
/// # enum UuidValidationError {
/// #     #[error("The provided UUID value is not a string")]
/// #     NotAString,
/// #
/// #     #[error("The provided UUID value is not correct")]
/// #     InvalidUuid,
/// # }
/// #
/// # impl Validator for UuidValidator {
/// #     fn validate(&self, value: &Value) -> Result<(), ValidationError> {
/// #         let Value::String(string) = value else {
/// #             return Err(UuidValidationError::NotAString.into());
/// #         };
/// #
/// #         Uuid::from_str(string)
/// #             .map_err(|_| UuidValidationError::InvalidUuid)?;
/// #
/// #         Ok(())
/// #     }
/// # }
/// #
/// # let validator = UuidValidator;
/// use json_fields::field::Field;
/// use json_fields::field::object_field::ObjectField;
/// use json_fields::field::custom_field::CustomField;
///
/// let object_field = Field::Object(ObjectField::from([
///     ("uuid", Field::CustomValidator(CustomField::new(validator)))
/// ]));
///
/// let incorrect_uuid = json!({ "uuid": "Hello world" });
/// let correct_uuid = json!({ "uuid": "550e8400-e29b-41d4-a716-446655440000" });
///
/// assert!(object_field.validate(&incorrect_uuid).is_err());
/// assert!(object_field.validate(&correct_uuid).is_ok());
/// ```
pub trait Validator {
    fn validate(&self, value: &Value) -> Result<(), ValidationError>;
}
