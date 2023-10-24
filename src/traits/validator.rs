use std::error::Error;
use serde_json::Value;

pub trait Validator {
    type E: Error;

    fn validate(&self, value: &Value) -> Result<(), Self::E>;
}
