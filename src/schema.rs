use crate::errors::validation_error::ValidationError;
use crate::field::Field;
use crate::{validator_impl, Validator};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Schema {
    root: Field,
}

#[validator_impl]
impl Validator for Schema {
    fn validate(&self, value: &Value) -> Result<(), ValidationError> {
        self.root.validate(value)
    }
}

impl From<Field> for Schema {
    fn from(value: Field) -> Self {
        Self { root: value }
    }
}
