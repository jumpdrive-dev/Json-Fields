use std::fmt::{Debug, Formatter};
use serde_json::Value;
use crate::errors::validation_error::ValidationError;
use crate::Validator;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize)
)]
pub struct CustomField(Box<dyn Validator>);

impl CustomField {
    pub fn new(validator: impl Validator + 'static) -> Self {
        CustomField(Box::new(validator))
    }
}

impl Validator for CustomField {
    fn validate(&self, value: &Value) -> Result<(), ValidationError> {
        todo!()
    }
}

impl Debug for CustomField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Custom validator>")
    }
}
