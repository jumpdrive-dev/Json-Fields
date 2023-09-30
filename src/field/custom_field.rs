use crate::errors::validation_error::ValidationError;
use crate::{Validator, validator_impl};
use serde_json::Value;
use std::fmt::{Debug, Formatter};
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
/// # use json_fields::{Deserialize, Serialize, Validator, validator_impl};
/// #
/// # #[derive(Serialize, Deserialize)]
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
/// # #[validator_impl]
/// # impl Validator for UuidValidator {
/// #     fn validate(&self, value: &Value) -> Result<(), ValidationError> {
/// #         let Value::String(string) = value else {
/// #             return Err(ValidationError::new_custom(UuidValidationError::NotAString));
/// #         };
/// #
/// #         Uuid::from_str(string)
/// #             .map_err(|_| ValidationError::new_custom(UuidValidationError::InvalidUuid))?;
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
#[derive(Serialize, Deserialize)]
pub struct CustomField(Box<dyn Validator>);

impl CustomField {
    pub fn new(validator: impl Validator + 'static) -> Self {
        CustomField(Box::new(validator))
    }
}

#[validator_impl]
impl Validator for CustomField {
    fn validate(&self, value: &Value) -> Result<(), ValidationError> {
        self.0.validate(value)
    }
}

impl Debug for CustomField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Custom validator>")
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::{Display, Formatter};
    use std::str::FromStr;
    use serde_json::{json, Value};
    use thiserror::Error;
    use uuid::Uuid;
    use crate::{Deserialize, Serialize, Validator, validator_impl};
    use crate::errors::validation_error::ValidationError;
    use crate::field::custom_field::CustomField;
    use crate::field::Field;
    use crate::field::object_field::ObjectField;

    #[derive(Debug, Error)]
    struct StrError(pub String);

    impl Display for StrError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl FromStr for StrError {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(StrError(s.to_string()))
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct UuidValidator;

    #[validator_impl]
    impl Validator for UuidValidator {
        fn validate(&self, value: &Value) -> Result<(), ValidationError> {
            let Value::String(string) = value else {
                return Err(ValidationError::new_custom(StrError::from_str("not a string").unwrap()));
            };

            Uuid::from_str(string)
                .map_err(|_| ValidationError::new_custom(StrError::from_str("invalid uuid").unwrap()))?;

            Ok(())
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct ExactStringValidator(String);

    #[validator_impl]
    impl Validator for ExactStringValidator {
        fn validate(&self, value: &Value) -> Result<(), ValidationError> {
            let Value::String(string) = value else {
                return Err(ValidationError::new_custom(StrError::from_str("not a string").unwrap()));
            };

            if string != &self.0 {
                return Err(ValidationError::new_custom(StrError::from_str("not a").unwrap()));
            }

            Ok(())
        }
    }

    #[test]
    fn custom_validator_can_be_used() {
        let incorrect_uuid = json!("Hello world");
        let correct_uuid = json!("550e8400-e29b-41d4-a716-446655440000");

        assert!(UuidValidator.validate(&incorrect_uuid).is_err());
        assert!(UuidValidator.validate(&correct_uuid).is_ok());
    }

    #[test]
    fn custom_validator_with_config_can_be_used() {
        let validator = ExactStringValidator("a".to_string());

        let incorrect_value = json!("b");
        let correct_value = json!("a");

        assert!(validator.validate(&incorrect_value).is_err());
        assert!(validator.validate(&correct_value).is_ok());
    }

    #[test]
    fn custom_validator_can_be_serialized() {
        let string_result = serde_json::to_string(&UuidValidator);
        assert!(string_result.is_ok());

        let string = string_result.unwrap();
        dbg!(&string);
        let deserialize_result = serde_json::from_str(&string);
        assert!(deserialize_result.is_ok());

        let uuid_validator: UuidValidator = deserialize_result.unwrap();
    }

    #[test]
    fn custom_validator_with_config_can_be_serialized() {
        let exact_validator = ExactStringValidator("a".to_string());

        let string_result = serde_json::to_string(&exact_validator);
        assert!(string_result.is_ok());

        let string = string_result.unwrap();
        dbg!(&string);
        let deserialize_result = serde_json::from_str(&string);
        assert!(deserialize_result.is_ok());

        let deserialized_validator: ExactStringValidator = deserialize_result.unwrap();
    }

    #[test]
    fn custom_field_can_is_serialized_correctly_from_within_another_field() {
        let exact_validator = ExactStringValidator("a".to_string());

        let object_field = Field::Object(ObjectField::from([
            ("uuid", Field::CustomValidator(CustomField::new(exact_validator)))
        ]));

        let string_result = serde_json::to_string(&object_field);
        assert!(string_result.is_ok());

        let string = string_result.unwrap();
        dbg!(&string);
        let deserialize_result = serde_json::from_str(&string);
        assert!(deserialize_result.is_ok());

        let deserialized_field: Field = deserialize_result.unwrap();
    }
}
