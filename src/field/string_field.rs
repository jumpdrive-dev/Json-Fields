use crate::errors::validation_error::ValidationError;
use crate::Validator;
use serde_json::Value;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StringField {
    min_length: Option<u32>,
    max_length: Option<u32>,
}

impl Validator for StringField {
    fn validate(&self, value: &Value) -> Result<(), ValidationError> {
        todo!()
    }
}
