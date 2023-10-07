use serde::{Deserialize, Serialize};
use crate::errors::validation_error::ValidationError;
use crate::field::Field;
use crate::{Validator, validator_impl};
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
