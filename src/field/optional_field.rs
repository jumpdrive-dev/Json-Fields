use crate::errors::validation_error::ValidationError;
use crate::field::Field;
use crate::{validator_impl, Validator};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct OptionalField {
    field: Box<Field>,
}

#[validator_impl]
impl Validator for OptionalField {
    fn validate(&self, _value: &Value) -> Result<(), ValidationError> {
        todo!()
    }
}
