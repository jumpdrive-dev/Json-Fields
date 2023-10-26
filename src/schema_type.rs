use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use crate::schema_type::advanced_type::{AdvancedType, AdvancedTypeValidationError};
use crate::schema_type::basic_type::{BasicType, BasicTypeValidationError};
use crate::traits::validator::Validator;

pub mod basic_type;
pub mod advanced_type;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
#[cfg_attr(test, derive(PartialEq))]
pub enum SchemaType {
    Basic(BasicType),
    Advanced(AdvancedType),
    Array(Vec<SchemaType>),
    Object(HashMap<String, SchemaType>),
}

#[derive(Debug, Error, PartialEq)]
pub enum SchemaTypeValidationError {
    #[error("{0}")]
    BasicTypeValidationError(#[from] BasicTypeValidationError),

    #[error("{0}")]
    AdvancedTypeValidationError(#[from] AdvancedTypeValidationError),
}

impl Validator for SchemaType {
    type E = SchemaTypeValidationError;

    fn validate(&self, value: &Value) -> Result<(), Self::E> {
        match self {
            SchemaType::Basic(basic_type) => Ok(basic_type.validate(value)?),
            SchemaType::Advanced(advanced_type) => Ok(advanced_type.validate(value)?),
            SchemaType::Array(_) => {
                todo!()
            }
            SchemaType::Object(_) => {
                todo!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use serde_json::json;
    use crate::schema_type::{AdvancedType, BasicType, SchemaType};
    use crate::schema_type::advanced_type::advanced_string_type::AdvancedStringType;

    #[test]
    fn basic_schema_type_can_be_deserialized() {
        let value: SchemaType = serde_json::from_value(json!("string"))
            .unwrap();

        assert_eq!(value, SchemaType::Basic(BasicType::String));
    }

    #[test]
    fn advanced_string_type_can_be_deserialized() {
        let value: SchemaType = serde_json::from_value(json!({ "$": "string", "minLength": 10 }))
            .unwrap();

        assert_eq!(value, SchemaType::Advanced(AdvancedType::String(AdvancedStringType {
            min_length: Some(10),
            ..Default::default()
        })));

        let result = serde_json::from_value::<SchemaType>(json!({ "minLength": 10 }));
        assert!(result.is_err());
    }

    #[test]
    fn nested_values_in_object_are_deserialized_correctly() {
        let value: SchemaType = serde_json::from_value(json!({
            "name": "string"
        }))
            .unwrap();

        assert_eq!(value, SchemaType::Object(HashMap::from([
            ("name".to_string(), SchemaType::Basic(BasicType::String))
        ])));
    }

    #[test]
    fn nested_values_in_array_are_deserialized_correctly() {
        let value: SchemaType = serde_json::from_value(json!([
            "string"
        ]))
            .unwrap();

        assert_eq!(value, SchemaType::Array(vec![
            SchemaType::Basic(BasicType::String)
        ]));
    }
}
