use serde::{Deserialize, Serialize};
use crate::errors::validation_error::ValidationError;
use crate::{Validator, validator_impl};
use serde_json::Value;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StringField {
    min_length: Option<u32>,
    max_length: Option<u32>,
}

#[validator_impl]
impl Validator for StringField {
    fn validate(&self, value: &Value) -> Result<(), ValidationError> {
        let Value::String(_) = value else {
            return Err(ValidationError::NotAString);
        };

        Ok(())
    }
}
