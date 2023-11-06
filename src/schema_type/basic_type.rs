use crate::traits::validator::Validator;
use serde::{Deserialize, Serialize};
use serde_email::is_valid_email;
use serde_json::{Number, Value};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum BasicTypeValidationError {
    #[error("Expected a filled string, but got an empty string")]
    EmptyString,

    #[error("Expected a positive number, but got '{0}'")]
    NotAPositiveNumber(Number),

    #[error("Expected a negative number, but got '{0}'")]
    NotANegativeNumber(Number),

    #[error("Expected a u8, but got '{0}'")]
    NotAU8(Number),

    #[error("Expected a u16, but got '{0}'")]
    NotAU16(Number),

    #[error("Expected a u32, but got '{0}'")]
    NotAU32(Number),

    #[error("Expected a u64, but got '{0}'")]
    NotAU64(Number),

    #[error("Expected a i8, but got '{0}'")]
    NotAI8(Number),

    #[error("Expected a i16, but got '{0}'")]
    NotAI16(Number),

    #[error("Expected a i32, but got '{0}'")]
    NotAI32(Number),

    #[error("Expected a i64, but got '{0}'")]
    NotAI64(Number),

    #[error("Incorrect type provided. Expected '{0}' but got '{1}'")]
    IncorrectType(BasicType, Value),

    #[error("Expected a UUID, but got '{0}'")]
    IncorrectUuid(String),

    #[error("Expected an email, but got '{0}'")]
    IncorrectEmail(String),
}

/// Basic types don't have any additional configuration and only check the variant of [Value] and
/// might do a bit of extra validation in the case of [BasicType::Uuid] and [BasicType::Email].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BasicType {
    /// Matches any value and does not do any additional validation.
    Any,

    /// Matches a `true` or `false` value.
    Boolean,

    /// Matches any string, this could also be an empty string.
    String,

    /// Matches any string that is not empty.
    FilledString,

    /// Matches any number.
    Number,

    /// Matches any positive number including 0.
    PositiveNumber,

    /// Matches any negative number including 0.
    NegativeNumber,

    /// Matches if the value is between 0 and [u8::MAX] (inclusive)
    U8,

    /// Matches if the value is between 0 and [u16::MAX] (inclusive)
    U16,

    /// Matches if the value is between 0 and [u32::MAX] (inclusive)
    U32,

    /// Matches if the value is between 0 and [u64::MAX] (inclusive)
    U64,

    /// Matches if the value is between [i8::MIN] and [i8::MAX] (inclusive)
    I8,

    /// Matches if the value is between [i16::MIN] and [i16::MAX] (inclusive)
    I16,

    /// Matches if the value is between [i32::MIN] and [i32::MAX] (inclusive)
    I32,

    /// Matches if the value is between [i64::MIN] and [i64::MAX] (inclusive)
    I64,

    /// Checks if the value is `null`.
    Null,

    /// Check if the value is any object. Use an object using `{}` to check for an object with
    /// the provided keys instead.
    Object,

    /// Check if the value is any array. Use an array using `[]` with a single type to check for
    /// an array that contains any number of the provided type, or an array with multiple types
    /// to check for a tuple with the exact same number of items.
    Array,

    /// Matches if the value is a string and is formatted as an [Uuid]. Check [Uuid::parse] for more
    /// details about formatting.
    Uuid,

    /// Matches if the value is a string and is formatted as an email address.
    Email,
}

impl BasicType {
    fn validate_number(&self, number: &Number) -> Result<(), BasicTypeValidationError> {
        match self {
            BasicType::PositiveNumber => {
                let value = number.as_f64()
                    .ok_or(BasicTypeValidationError::NotAPositiveNumber(number.clone()))?;

                if value < 0_f64 {
                    return Err(BasicTypeValidationError::NotAPositiveNumber(number.clone()))
                }

                Ok(())
            },
            BasicType::NegativeNumber => {
                let value = number.as_f64()
                    .ok_or(BasicTypeValidationError::NotANegativeNumber(number.clone()))?;

                if value > 0_f64 {
                    return Err(BasicTypeValidationError::NotANegativeNumber(number.clone()));
                }

                Ok(())
            },
            BasicType::U8 => {
                let value = number.as_u64()
                    .ok_or(BasicTypeValidationError::NotAU8(number.clone()))?;

                if value > 255 {
                    return Err(BasicTypeValidationError::NotAU8(number.clone()));
                }

                Ok(())
            },
            BasicType::U16 => {
                let value = number.as_u64()
                    .ok_or(BasicTypeValidationError::NotAU16(number.clone()))?;

                if value > 65535 {
                    return Err(BasicTypeValidationError::NotAU16(number.clone()));
                }

                Ok(())
            },
            BasicType::U32 => {
                let value = number.as_u64()
                    .ok_or(BasicTypeValidationError::NotAU32(number.clone()))?;

                if value > 4294967295 {
                    return Err(BasicTypeValidationError::NotAU32(number.clone()));
                }

                Ok(())
            },
            BasicType::U64 => {
                if !number.is_u64() {
                    return Err(BasicTypeValidationError::NotAU64(number.clone()));
                }

                Ok(())
            },
            BasicType::I8 => {
                let value = number.as_i64()
                    .ok_or(BasicTypeValidationError::NotAI8(number.clone()))?;
            },
            BasicType::I16 => todo!(),
            BasicType::I32 => todo!(),
            BasicType::I64 => todo!(),
            _ => unreachable!(),
        }
    }
}

impl Display for BasicType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let slice = match self {
            BasicType::Any => "any",
            BasicType::String => "string",
            BasicType::FilledString => "filled string",
            BasicType::Number => "number",
            BasicType::Null => "null",
            BasicType::Object => "object",
            BasicType::Array => "array",
            BasicType::Uuid => "uuid",
            BasicType::Email => "email",
            BasicType::Boolean => "boolean",
            BasicType::PositiveNumber => "positive number",
            BasicType::NegativeNumber => "negative number",
            BasicType::U8 => "u8",
            BasicType::U16 => "u16",
            BasicType::U32 => "u32",
            BasicType::U64 => "u64",
            BasicType::I8 => "i8",
            BasicType::I16 => "i16",
            BasicType::I32 => "i32",
            BasicType::I64 => "i64",
        };

        write!(f, "{}", slice)
    }
}

impl Validator for BasicType {
    type E = BasicTypeValidationError;

    fn validate(&self, value: &Value) -> Result<(), Self::E> {
        match (self, value) {
            (BasicType::Any, _) => Ok(()),
            (BasicType::Null, Value::Null) => Ok(()),
            (BasicType::Boolean, Value::Bool(_)) => Ok(()),
            (BasicType::Number, Value::Number(_)) => Ok(()),
            (
                BasicType::PositiveNumber
                | BasicType::NegativeNumber
                | BasicType::U8
                | BasicType::U16
                | BasicType::U32
                | BasicType::U64
                | BasicType::I8
                | BasicType::I16
                | BasicType::I32
                | BasicType::I64,
                Value::Number(number)
            ) => self.validate_number(number),
            (BasicType::String, Value::String(_)) => Ok(()),
            (BasicType::FilledString, Value::String(string)) => {
                if string.is_empty() {
                    return Err(BasicTypeValidationError::EmptyString);
                }

                Ok(())
            },
            (BasicType::Array, Value::Array(_)) => Ok(()),
            (BasicType::Object, Value::Object(_)) => Ok(()),
            (BasicType::Uuid, Value::String(string_value)) => match Uuid::from_str(string_value) {
                Ok(_) => Ok(()),
                Err(_) => Err(BasicTypeValidationError::IncorrectUuid(
                    string_value.to_string(),
                )),
            },
            (BasicType::Email, Value::String(string_value)) => {
                if !is_valid_email(string_value) {
                    return Err(BasicTypeValidationError::IncorrectEmail(
                        string_value.to_string(),
                    ));
                }

                Ok(())
            }
            (_, _) => Err(BasicTypeValidationError::IncorrectType(
                self.clone(),
                value.clone(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::schema_type::basic_type::BasicType;
    use crate::traits::validator::Validator;
    use serde_json::{json, Number, Value};

    #[test]
    fn any_type_is_validated_correctly() {
        assert!(BasicType::Any.validate(&json!(null)).is_ok());
        assert!(BasicType::Any.validate(&json!("")).is_ok());
        assert!(BasicType::Any.validate(&json!(10)).is_ok());
        assert!(BasicType::Any.validate(&json!(true)).is_ok());
        assert!(BasicType::Any.validate(&json!(false)).is_ok());
        assert!(BasicType::Any.validate(&json!([])).is_ok());
        assert!(BasicType::Any.validate(&json!({})).is_ok());
    }

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
        assert!(BasicType::String
            .validate(&json!("f1df9904-6f6b-4157-8a82-b1a566a50ec2"))
            .is_ok());
        assert!(BasicType::String
            .validate(&json!("alice@example.com"))
            .is_ok());

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
        assert!(BasicType::Object
            .validate(&json!({ "name": "Alice" }))
            .is_ok());
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
        assert!(BasicType::Uuid
            .validate(&json!("f1df9904-6f6b-4157-8a82-b1a566a50ec2"))
            .is_ok());

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
        assert!(BasicType::Email
            .validate(&json!("alice@example.com"))
            .is_ok());
        assert!(BasicType::Email
            .validate(&json!("alice+alias@example.com"))
            .is_ok());

        assert!(BasicType::Email
            .validate(&json!("f1df9904-6f6b-4157-8a82-b1a566a50ec2"))
            .is_err());
        assert!(BasicType::Email.validate(&json!("")).is_err());
        assert!(BasicType::Email.validate(&json!(null)).is_err());
        assert!(BasicType::Email.validate(&json!(true)).is_err());
        assert!(BasicType::Email.validate(&json!(false)).is_err());
        assert!(BasicType::Email.validate(&json!(10)).is_err());
        assert!(BasicType::Email.validate(&json!([])).is_err());
        assert!(BasicType::Email.validate(&json!({})).is_err());
    }

    #[test]
    fn positive_number_is_validated_correctly() {
        assert!(BasicType::PositiveNumber.validate(&json!("")).is_err());
        assert!(BasicType::PositiveNumber.validate(&json!(0)).is_ok());
        assert!(BasicType::PositiveNumber.validate(&json!(1)).is_ok());
        assert!(BasicType::PositiveNumber.validate(&json!(1.1)).is_ok());
        assert!(BasicType::PositiveNumber.validate(&json!(-1)).is_err());
        assert!(BasicType::PositiveNumber.validate(&json!(-1.1)).is_err());
    }

    #[test]
    fn negative_number_is_validated_correctly() {
        assert!(BasicType::NegativeNumber.validate(&json!("")).is_err());
        assert!(BasicType::NegativeNumber.validate(&json!(0)).is_ok());
        assert!(BasicType::NegativeNumber.validate(&json!(-1)).is_ok());
        assert!(BasicType::NegativeNumber.validate(&json!(-1.1)).is_ok());
        assert!(BasicType::NegativeNumber.validate(&json!(1)).is_err());
        assert!(BasicType::NegativeNumber.validate(&json!(1.1)).is_err());
    }

    #[test]
    fn u8_is_validated_correctly() {
        assert!(BasicType::U8.validate(&json!("")).is_err());
        assert!(BasicType::U8.validate(&json!(0)).is_ok());
        assert!(BasicType::U8.validate(&json!(255)).is_ok());
        assert!(BasicType::U8.validate(&json!(-1)).is_err());
        assert!(BasicType::U8.validate(&json!(256)).is_err());
        assert!(BasicType::U8.validate(&json!(1.1)).is_err());
    }

    #[test]
    fn u16_is_validated_correctly() {
        assert!(BasicType::U16.validate(&json!("")).is_err());
        assert!(BasicType::U16.validate(&json!(0)).is_ok());
        assert!(BasicType::U16.validate(&json!(65535)).is_ok());
        assert!(BasicType::U16.validate(&json!(-1)).is_err());
        assert!(BasicType::U16.validate(&json!(65536)).is_err());
        assert!(BasicType::U16.validate(&json!(1.1)).is_err());
    }

    #[test]
    fn u32_is_validated_correctly() {
        assert!(BasicType::U32.validate(&json!("")).is_err());
        assert!(BasicType::U32.validate(&json!(0)).is_ok());
        assert!(BasicType::U32.validate(&json!(4294967295_u64)).is_ok());
        assert!(BasicType::U32.validate(&json!(-1)).is_err());
        assert!(BasicType::U32.validate(&json!(4294967296_u64)).is_err());
        assert!(BasicType::U32.validate(&json!(1.1)).is_err());
    }

    #[test]
    fn u64_is_validated_correctly() {
        assert!(BasicType::U64.validate(&json!("")).is_err());
        assert!(BasicType::U64.validate(&json!(0)).is_ok());
        assert!(BasicType::U64.validate(&json!(18446744073709551615_u64)).is_ok());
        assert!(BasicType::U64.validate(&json!(-1)).is_err());
        assert!(BasicType::U64.validate(&json!(1.1)).is_err());
    }

    #[test]
    fn i8_is_validated_correctly() {
        assert!(BasicType::I8.validate(&json!("")).is_err());
        assert!(BasicType::I8.validate(&json!(0)).is_ok());
        assert!(BasicType::I8.validate(&json!(127)).is_ok());
        assert!(BasicType::I8.validate(&json!(-128)).is_ok());
        assert!(BasicType::I8.validate(&json!(128)).is_err());
        assert!(BasicType::I8.validate(&json!(-129)).is_err());
        assert!(BasicType::I8.validate(&json!(1.1)).is_err());
    }

    #[test]
    fn i16_is_validated_correctly() {
        assert!(BasicType::I16.validate(&json!("")).is_err());
        assert!(BasicType::I16.validate(&json!(0)).is_ok());
        assert!(BasicType::I16.validate(&json!(32767)).is_ok());
        assert!(BasicType::I16.validate(&json!(-32768)).is_ok());
        assert!(BasicType::I16.validate(&json!(32768)).is_err());
        assert!(BasicType::I16.validate(&json!(-32769)).is_err());
        assert!(BasicType::I16.validate(&json!(1.1)).is_err());
    }

    #[test]
    fn i32_is_validated_correctly() {
        assert!(BasicType::I32.validate(&json!("")).is_err());
        assert!(BasicType::I32.validate(&json!(0)).is_ok());
        assert!(BasicType::I32.validate(&json!(2147483647)).is_ok());
        assert!(BasicType::I32.validate(&json!(-2147483648)).is_ok());
        assert!(BasicType::I32.validate(&json!(2147483648_i64)).is_err());
        assert!(BasicType::I32.validate(&json!(-2147483649_i64)).is_err());
        assert!(BasicType::I32.validate(&json!(1.1)).is_err());
    }

    #[test]
    fn i64_is_validated_correctly() {
        assert!(BasicType::I64.validate(&json!("")).is_err());
        assert!(BasicType::I64.validate(&json!(0)).is_ok());
        assert!(BasicType::I64.validate(&json!(9223372036854775807_i128)).is_ok());
        assert!(BasicType::I64.validate(&json!(-9223372036854775808_i128)).is_ok());
        assert!(BasicType::I64.validate(&json!(9223372036854775808_i128)).is_err());
        assert!(BasicType::I64.validate(&json!(-9223372036854775809_i128)).is_err());
        assert!(BasicType::I64.validate(&json!(1.1)).is_err());
    }
}
