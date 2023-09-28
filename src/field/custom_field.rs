use std::fmt::{Debug, Formatter};
use serde_json::Value;
use crate::errors::validation_error::ValidationError;
use crate::Validator;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Custom fields allow you to use your own validators within other fields, like an `ObjectField`
/// as shown here:
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
#[cfg_attr(
feature = "serde",
derive(Serialize, Deserialize)
)]
pub struct CustomField(Box<dyn Validator>);

impl CustomField {
    pub fn new(validator: impl Validator + 'static) -> Self {
        CustomField(Box::new(validator))
    }
}

impl Validator for CustomField {
    fn validate(&self, value: &Value) -> Result<(), ValidationError> {
        todo!()
    }
}

impl Debug for CustomField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Custom validator>")
    }
}
