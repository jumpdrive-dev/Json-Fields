use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use crate::schema_type::advanced_type::AdvancedType;
use crate::schema_type::{SchemaType, SchemaTypeValidationError};
use crate::traits::validator::Validator;

#[derive(Debug, PartialEq, Error)]
pub enum ObjectTypeError {
    #[error("Expected an object, but got something else")]
    NotAnObject,

    #[error("Missing object key: '{0}'")]
    MissingObjectKey(String),

    #[error("{0}")]
    SchemaTypeValidationError(Box<SchemaTypeValidationError>),
}

impl From<SchemaTypeValidationError> for ObjectTypeError {
    fn from(value: SchemaTypeValidationError) -> Self {
        ObjectTypeError::SchemaTypeValidationError(Box::new(value))
    }
}

/// This type checks for the exact keys. This differs from [SchemaType::Object] in that it cannot
/// resolve to any other advanced type, so it allows for '$' or '?' to be used for something else.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ObjectType {
    pub object: HashMap<String, SchemaType>,
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "object")
    }
}

impl From<HashMap<String, SchemaType>> for ObjectType {
    fn from(value: HashMap<String, SchemaType>) -> Self {
        ObjectType {
            object: value
        }
    }
}

impl Validator for ObjectType {
    type E = ObjectTypeError;

    fn validate(&self, value: &Value) -> Result<(), Self::E> {
        let Value::Object(target_map) = value else {
            return Err(ObjectTypeError::NotAnObject);
        };

        for (key, schema) in &self.object {
            let Some(value) = target_map.get(key) else {
                if let SchemaType::Advanced(AdvancedType::Optional(_)) = schema {
                    return Ok(());
                };

                return Err(ObjectTypeError::MissingObjectKey(key.to_string()));
            };

            schema.validate(value)?;
        }

        Ok(())
    }
}
