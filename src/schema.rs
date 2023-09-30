use serde::{Deserialize, Serialize};
use crate::errors::validation_error::ValidationError;
use crate::field::Field;
use serde_json::Value;
use crate::{Validator, validator_impl};

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
