use std::fmt::{Display, Formatter};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use serde_email::{Email, is_valid_email};
use serde_json::Value;
use thiserror::Error;
use uuid::{Error, Uuid};
use crate::traits::validator::Validator;

#[derive(Debug, Error)]
pub enum BasicTypeValidationError {
    #[error("Incorrect type provided. Expected '{0}' but got '{1}'")]
    IncorrectType(BasicType, Value),

    #[error("Expected a UUID, but got '{0}'")]
    IncorrectUuid(String),

    #[error("Expected an email, but got '{0}'")]
    IncorrectEmail(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(PartialEq))]
pub enum BasicType {
    String,
    Number,
    Null,
    Object,
    Array,
    Uuid,
    Email,
}

impl Display for BasicType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let slice = match self {
            BasicType::String => "string",
            BasicType::Number => "number",
            BasicType::Null => "null",
            BasicType::Object => "object",
            BasicType::Array => "array",
            BasicType::Uuid => "uuid",
            BasicType::Email => "email",
        };

        write!(f, "{}", slice)
    }
}

impl Validator for BasicType {
    type E = BasicTypeValidationError;

    fn validate(&self, value: &Value) -> Result<(), Self::E> {
        match (self, value) {
            (BasicType::Null, Value::Null) => Ok(()),
            (BasicType::Number, Value::Number(_)) => Ok(()),
            (BasicType::String, Value::String(_)) => Ok(()),
            (BasicType::Array, Value::Array(_)) => Ok(()),
            (BasicType::Object, Value::Object(_)) => Ok(()),
            (BasicType::Uuid, Value::String(string_value)) => {
                match Uuid::from_str(&string_value) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(BasicTypeValidationError::IncorrectUuid(string_value.to_string())),
                }
            },
            (BasicType::Email, Value::String(string_value)) => {
                if !is_valid_email(string_value) {
                    return Err(BasicTypeValidationError::IncorrectEmail(string_value.to_string()));
                }

                Ok(())
            },
            (_, _) => Err(BasicTypeValidationError::IncorrectType(self.clone(), value.clone())),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::schema_type::basic_type::BasicType;
    use crate::traits::validator::Validator;

    #[test]
    fn basic_null_type_is_validated_correctly() {
        assert!(BasicType::Null.validate(&json!(null)).is_ok());

        assert!(BasicType::Null.validate(&json!("")).is_err());
        assert!(BasicType::Null.validate(&json!(10)).is_err());
        assert!(BasicType::Null.validate(&json!(true)).is_err());
        assert!(BasicType::Null.validate(&json!(false)).is_err());
        assert!(BasicType::Null.validate(&json!([])).is_err());
        assert!(BasicType::Null.validate(&json!({})).is_err());
    }

    #[test]
    fn basic_string_type_is_validated_correctly() {
        assert!(BasicType::String.validate(&json!("")).is_ok());
        assert!(BasicType::String.validate(&json!("f1df9904-6f6b-4157-8a82-b1a566a50ec2")).is_ok());
        assert!(BasicType::String.validate(&json!("alice@example.com")).is_ok());

        assert!(BasicType::String.validate(&json!(null)).is_err());
        assert!(BasicType::String.validate(&json!(true)).is_err());
        assert!(BasicType::String.validate(&json!(false)).is_err());
        assert!(BasicType::String.validate(&json!(10)).is_err());
        assert!(BasicType::String.validate(&json!([])).is_err());
        assert!(BasicType::String.validate(&json!({})).is_err());
    }

    #[test]
    fn basic_number_type_is_validated_correctly() {
        assert!(BasicType::Number.validate(&json!(10)).is_ok());

        assert!(BasicType::Number.validate(&json!(null)).is_err());
        assert!(BasicType::Number.validate(&json!("")).is_err());
        assert!(BasicType::Number.validate(&json!(true)).is_err());
        assert!(BasicType::Number.validate(&json!(false)).is_err());
        assert!(BasicType::Number.validate(&json!([])).is_err());
        assert!(BasicType::Number.validate(&json!({})).is_err());
    }

    #[test]
    fn basic_object_type_is_validated_correctly() {
        assert!(BasicType::Object.validate(&json!({})).is_ok());
        assert!(BasicType::Object.validate(&json!({ "name": "Alice" })).is_ok());
        assert!(BasicType::Object.validate(&json!({ "age": 10 })).is_ok());

        assert!(BasicType::Object.validate(&json!(null)).is_err());
        assert!(BasicType::Object.validate(&json!(10)).is_err());
        assert!(BasicType::Object.validate(&json!("")).is_err());
        assert!(BasicType::Object.validate(&json!(true)).is_err());
        assert!(BasicType::Object.validate(&json!(false)).is_err());
        assert!(BasicType::Object.validate(&json!([])).is_err());
    }

    #[test]
    fn basic_array_type_is_validated_correctly() {
        assert!(BasicType::Array.validate(&json!([])).is_ok());
        assert!(BasicType::Array.validate(&json!(["Alice"])).is_ok());
        assert!(BasicType::Array.validate(&json!([10])).is_ok());

        assert!(BasicType::Array.validate(&json!(null)).is_err());
        assert!(BasicType::Array.validate(&json!(10)).is_err());
        assert!(BasicType::Array.validate(&json!("")).is_err());
        assert!(BasicType::Array.validate(&json!(true)).is_err());
        assert!(BasicType::Array.validate(&json!(false)).is_err());
        assert!(BasicType::Array.validate(&json!({})).is_err());
    }

    #[test]
    fn basic_uuid_type_is_validated_correctly() {
        assert!(BasicType::Uuid.validate(&json!("f1df9904-6f6b-4157-8a82-b1a566a50ec2")).is_ok());

        assert!(BasicType::Uuid.validate(&json!("")).is_err());
        assert!(BasicType::Uuid.validate(&json!(null)).is_err());
        assert!(BasicType::Uuid.validate(&json!(true)).is_err());
        assert!(BasicType::Uuid.validate(&json!(false)).is_err());
        assert!(BasicType::Uuid.validate(&json!(10)).is_err());
        assert!(BasicType::Uuid.validate(&json!([])).is_err());
        assert!(BasicType::Uuid.validate(&json!({})).is_err());
    }

    #[test]
    fn basic_email_type_is_validated_correctly() {
        assert!(BasicType::Email.validate(&json!("alice@example.com")).is_ok());

        assert!(BasicType::Email.validate(&json!("f1df9904-6f6b-4157-8a82-b1a566a50ec2")).is_err());
        assert!(BasicType::Email.validate(&json!("")).is_err());
        assert!(BasicType::Email.validate(&json!(null)).is_err());
        assert!(BasicType::Email.validate(&json!(true)).is_err());
        assert!(BasicType::Email.validate(&json!(false)).is_err());
        assert!(BasicType::Email.validate(&json!(10)).is_err());
        assert!(BasicType::Email.validate(&json!([])).is_err());
        assert!(BasicType::Email.validate(&json!({})).is_err());
    }
}
