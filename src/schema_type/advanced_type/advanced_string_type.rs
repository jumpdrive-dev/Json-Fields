use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use crate::traits::validator::Validator;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(PartialEq))]
pub struct AdvancedStringType {
    pub require_filled: Option<bool>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
}

#[derive(Debug, Error)]
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

impl Validator for AdvancedStringType {
    type E = StringValidationError;

    fn validate(&self, value: &Value) -> Result<(), Self::E> {
        let Value::String(string) = value else {
            return Err(StringValidationError::NotAString);
        };

        if string.is_empty() && self.require_filled.unwrap_or(true) {
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
