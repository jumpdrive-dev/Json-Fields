pub mod advanced_string_type;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use crate::schema_type::advanced_type::advanced_string_type::{AdvancedStringType, StringValidationError};
use crate::schema_type::SchemaType;
use crate::traits::validator::Validator;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "$", rename_all = "camelCase")]
#[cfg_attr(test, derive(PartialEq))]
pub enum AdvancedType {
    String(AdvancedStringType),
    Optional(Box<SchemaType>),
}

#[derive(Debug, PartialEq, Error)]
pub enum AdvancedTypeValidationError {
    #[error("{0}")]
    StringValidationError(#[from] StringValidationError),
}

impl Validator for AdvancedType {
    type E = AdvancedTypeValidationError;

    fn validate(&self, value: &Value) -> Result<(), Self::E> {
        match self {
            AdvancedType::String(advanced_string) => Ok(advanced_string.validate(value)?),
            AdvancedType::Optional(nested_type) => {
                if let Value::Null = value {
                    return Ok(())
                }

                todo!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::schema_type::advanced_type::AdvancedType;
    use crate::schema_type::basic_type::BasicType;
    use crate::schema_type::SchemaType;

    #[test]
    fn optional_advanced_type_is_resolved_correctly() {
        let advanced_type = AdvancedType::Optional(Box::new(SchemaType::Basic(BasicType::String)));
    }
}
