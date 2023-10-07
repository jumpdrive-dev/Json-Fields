use serde::{Deserialize, Serialize};
use crate::errors::validation_error::ValidationError;
use crate::{Validator, validator_impl};
use serde_json::Value;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StringField {
    require_filled: Option<bool>,
    min_length: Option<usize>,
    max_length: Option<usize>,
}

#[validator_impl]
impl Validator for StringField {
    fn validate(&self, value: &Value) -> Result<(), ValidationError> {
        let Value::String(string) = value else {
            return Err(ValidationError::NotAString);
        };

        let require_filled = self.require_filled.unwrap_or(false);
        if require_filled && string.is_empty() {
            return Err(ValidationError::StringNotFilled);
        }

        if let Some(min_length) = self.min_length {
            if string.len() < min_length {
                return Err(ValidationError::StringNotMinLength(min_length, string.len()));
            }
        }

        if let Some(max_length) = self.max_length {
            if string.len() > max_length {
                return Err(ValidationError::StringExceedsMaxLength(max_length, string.len()));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::errors::validation_error::ValidationError;
    use crate::field::string_field::StringField;
    use crate::Validator;

    #[test]
    fn filled_check_is_checked_correctly() {
        let string_field = StringField {
            require_filled: Some(true),
            ..StringField::default()
        };

        let success = string_field.validate(&json!("abc"));
        let failure = string_field.validate(&json!(""));

        assert!(success.is_ok());
        assert!(matches!(failure, Err(ValidationError::StringNotFilled)));
    }

    #[test]
    fn min_length_check_is_checked_correctly() {
        let string_field = StringField {
            min_length: Some(3),
            ..StringField::default()
        };

        let success = string_field.validate(&json!("abc"));
        assert!(success.is_ok());

        let success = string_field.validate(&json!("abcdef"));
        assert!(success.is_ok());

        let failure = string_field.validate(&json!("ab"));

        assert!(matches!(failure, Err(ValidationError::StringNotMinLength(3, 2))));
    }

    #[test]
    fn max_length_check_is_checked_correctly() {
        let string_field = StringField {
            max_length: Some(6),
            ..StringField::default()
        };

        let success = string_field.validate(&json!("abc"));
        assert!(success.is_ok());

        let success = string_field.validate(&json!("abcdef"));
        assert!(success.is_ok());

        let failure = string_field.validate(&json!("abcdefg"));

        assert!(matches!(failure, Err(ValidationError::StringExceedsMaxLength(6, 7))));
    }
}
