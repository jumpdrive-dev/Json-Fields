use crate::errors::validation_error::ValidationError;
use crate::field::Field;
use crate::Validator;
use serde_json::Value;

#[derive(Debug)]
pub struct Schema {
    root: Field,
}

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
