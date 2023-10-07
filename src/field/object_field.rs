use crate::errors::validation_error::ValidationError;
use crate::field::Field;
use crate::{Validator, validator_impl};
use serde_json::Value;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectField(HashMap<String, Field>);

impl<const N: usize> From<[(String, Field); N]> for ObjectField {
    fn from(value: [(String, Field); N]) -> Self {
        ObjectField(HashMap::from(value))
    }
}

impl<const N: usize> From<[(&str, Field); N]> for ObjectField {
    fn from(value: [(&str, Field); N]) -> Self {
        let mut map = HashMap::new();

        for (key, field) in value {
            map.insert(key.to_string(), field);
        }

        ObjectField(map)
    }
}

#[validator_impl]
impl Validator for ObjectField {
    fn validate(&self, value: &Value) -> Result<(), ValidationError> {
        let Value::Object(map) = value else {
            return Err(ValidationError::NotAnObject);
        };

        for (key, field) in self.0.iter() {
            let value = map
                .get(key)
                .ok_or(ValidationError::MissingKeyInObject(key.to_string()))?;

            field.validate(value)?;
        }

        Ok(())
    }
}
