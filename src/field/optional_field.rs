use crate::errors::validation_error::ValidationError;
use crate::field::Field;
use crate::Validator;
use serde_json::Value;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OptionalField {
    field: Box<Field>,
}

impl Validator for OptionalField {
    fn validate(&self, value: &Value) -> Result<(), ValidationError> {
        todo!()
    }
}
