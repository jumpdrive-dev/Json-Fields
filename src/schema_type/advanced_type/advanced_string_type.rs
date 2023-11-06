use crate::traits::validator::Validator;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Display, Formatter};
use thiserror::Error;
use crate::shared::default_true;

#[derive(Debug, PartialEq, Error)]
pub enum StringValidationError {
    #[error("The provided value is not a string")]
    NotAString,

    #[error("The provided string is empty, but should be filled")]
    RequireFilled,

    #[error("The provided string is too long")]
    StringTooLong,

    #[error("The provided string is too short")]
    StringTooShort,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdvancedStringType {
    /// If this is set to true, the string cannot have a length of zero.
    #[serde(default = "default_true")]
    pub require_filled: bool,

    /// If set, ensures the string is at least n number of characters long.
    pub min_length: Option<usize>,

    /// If set, ensures the string is less of equal to n characters long.
    pub max_length: Option<usize>,
}

impl Display for AdvancedStringType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.require_filled {
            write!(f, "filled ")?;
        }

        write!(f, "string")
    }
}

impl Default for AdvancedStringType {
    fn default() -> Self {
        Self {
            require_filled: true,
            min_length: None,
            max_length: None,
        }
    }
}

impl Validator for AdvancedStringType {
    type E = StringValidationError;

    fn validate(&self, value: &Value) -> Result<(), Self::E> {
        let Value::String(string) = value else {
            return Err(StringValidationError::NotAString);
        };

        if string.is_empty() && self.require_filled {
            return Err(StringValidationError::RequireFilled);
        }

        if let Some(max_length) = self.max_length {
            if string.len() > max_length {
                return Err(StringValidationError::StringTooLong);
            }
        }

        if let Some(min_length) = self.min_length {
            if string.len() < min_length {
                return Err(StringValidationError::StringTooShort);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::schema_type::advanced_type::advanced_string_type::{
        AdvancedStringType, StringValidationError,
    };
    use crate::traits::validator::Validator;
    use serde_json::json;

    #[test]
    fn advanced_string_type_requires_filled_string_by_default() {
        assert_eq!(
            AdvancedStringType::default().validate(&json!("")),
            Err(StringValidationError::RequireFilled)
        );
    }

    #[test]
    fn advanced_string_type_requires_filled_is_checked_correctly() {
        assert_eq!(
            AdvancedStringType {
                require_filled: true,
                ..AdvancedStringType::default()
            }
            .validate(&json!("")),
            Err(StringValidationError::RequireFilled)
        );

        assert_eq!(
            AdvancedStringType {
                require_filled: false,
                ..AdvancedStringType::default()
            }
            .validate(&json!("")),
            Ok(())
        );
    }

    #[test]
    fn advanced_string_type_min_length_is_checked_correctly() {
        assert_eq!(
            AdvancedStringType {
                require_filled: false,
                min_length: Some(0),
                ..AdvancedStringType::default()
            }
            .validate(&json!("")),
            Ok(())
        );

        assert_eq!(
            AdvancedStringType {
                min_length: Some(5),
                ..AdvancedStringType::default()
            }
            .validate(&json!("abcde")),
            Ok(())
        );

        assert_eq!(
            AdvancedStringType {
                min_length: Some(5),
                ..AdvancedStringType::default()
            }
            .validate(&json!("abcdef")),
            Ok(())
        );

        assert_eq!(
            AdvancedStringType {
                min_length: Some(5),
                ..AdvancedStringType::default()
            }
            .validate(&json!("abcd")),
            Err(StringValidationError::StringTooShort)
        );
    }

    #[test]
    fn advanced_string_type_max_length_is_checked_correctly() {
        assert_eq!(
            AdvancedStringType {
                require_filled: false,
                max_length: Some(0),
                ..AdvancedStringType::default()
            }
            .validate(&json!("")),
            Ok(())
        );

        assert_eq!(
            AdvancedStringType {
                max_length: Some(5),
                ..AdvancedStringType::default()
            }
            .validate(&json!("abcde")),
            Ok(())
        );

        assert_eq!(
            AdvancedStringType {
                max_length: Some(5),
                ..AdvancedStringType::default()
            }
            .validate(&json!("abcd")),
            Ok(())
        );

        assert_eq!(
            AdvancedStringType {
                max_length: Some(5),
                ..AdvancedStringType::default()
            }
            .validate(&json!("abcdef")),
            Err(StringValidationError::StringTooLong)
        );
    }
}
