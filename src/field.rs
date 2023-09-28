use crate::errors::validation_error::ValidationError;
use crate::field::custom_field::CustomField;
use crate::field::object_field::ObjectField;
use crate::field::optional_field::OptionalField;
use crate::field::string_field::StringField;
use crate::Validator;
use serde_json::Value;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod custom_field;
pub mod object_field;
pub mod optional_field;
pub mod string_field;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Field {
    Optional(OptionalField),
    String(StringField),
    Object(ObjectField),
    CustomValidator(CustomField),
}

impl Validator for Field {
    fn validate(&self, value: &Value) -> Result<(), ValidationError> {
        match self {
            Field::Optional(optional_field) => optional_field.validate(value),
            Field::String(string_field) => string_field.validate(value),
            Field::Object(object_field) => object_field.validate(value),
            Field::CustomValidator(validator) => validator.validate(value),
        }
    }
}
