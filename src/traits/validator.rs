use serde_json::Value;
use std::error::Error;

pub trait Validator {
    type E: Error;

    fn validate(&self, value: &Value) -> Result<(), Self::E>;
}
