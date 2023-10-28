use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use crate::schema_type::advanced_type::{AdvancedType, AdvancedTypeValidationError};
use crate::schema_type::basic_type::{BasicType, BasicTypeValidationError};
use crate::traits::validator::Validator;

pub mod basic_type;
pub mod advanced_type;

/// Root schema type that encompasses all the different types that can be validated.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum SchemaType {
    Basic(BasicType),
    Advanced(AdvancedType),
    Array((Box<SchemaType>,)),
    Tuple(Vec<SchemaType>),
    Object(HashMap<String, SchemaType>),
}

impl Display for SchemaType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SchemaType::Basic(basic_type) => Display::fmt(basic_type, f),
            SchemaType::Advanced(advanced_type) => Display::fmt(advanced_type, f),
            SchemaType::Array(item) => {
                write!(f, "array filled with '{}'", item.0.to_string())
            }
            SchemaType::Tuple(_) => {
                todo!()
            }
            SchemaType::Object(_) => {
                todo!()
            }
        }
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum SchemaTypeValidationError {
    #[error("{0}")]
    BasicTypeValidationError(#[from] BasicTypeValidationError),

    #[error("{0}")]
    AdvancedTypeValidationError(#[from] AdvancedTypeValidationError),

    #[error("Expected an object, but got something else")]
    NotAnObject,

    #[error("Missing object key: '{0}'")]
    MissingObjectKey(String),
}

impl Validator for SchemaType {
    type E = SchemaTypeValidationError;

    fn validate(&self, value: &Value) -> Result<(), Self::E> {
        match self {
            SchemaType::Basic(basic_type) => Ok(basic_type.validate(value)?),
            SchemaType::Advanced(advanced_type) => Ok(advanced_type.validate(value)?),
            SchemaType::Array(items) => {
                todo!()
            }
            SchemaType::Tuple(items) => {
                todo!()
            }
            SchemaType::Object(map) => {
                let Value::Object(target_map) = value else {
                    return Err(SchemaTypeValidationError::NotAnObject);
                };

                for (key, schema) in map {
                    let Some(value) = target_map.get(key) else {
                        if let SchemaType::Advanced(AdvancedType::Optional(_)) = schema {
                            return Ok(());
                        };

                        return Err(SchemaTypeValidationError::MissingObjectKey(key.to_string()));
                    };

                    schema.validate(value)?;
                }

                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use serde_json::json;
    use crate::schema_type::{AdvancedType, BasicType, SchemaType, SchemaTypeValidationError};
    use crate::schema_type::advanced_type::advanced_string_type::AdvancedStringType;
    use crate::schema_type::basic_type::BasicTypeValidationError;
    use crate::traits::validator::Validator;

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

        assert_eq!(value, SchemaType::Array((
            Box::new(SchemaType::Basic(BasicType::String)),
        )));
    }

    #[test]
    fn nested_values_in_tuple_are_deserialized_correctly() {
        let value: SchemaType = serde_json::from_value(json!([
            "string",
            "number"
        ]))
            .unwrap();

        assert_eq!(value, SchemaType::Tuple(vec![
            SchemaType::Basic(BasicType::String),
            SchemaType::Basic(BasicType::Number),
        ]));
    }

    #[test]
    fn objects_are_validated_correctly() {
        let value: SchemaType = serde_json::from_value(json!({
            "name": "string",
            "age": "number",
        }))
            .unwrap();

        assert_eq!(value.validate(&json!({
            "name": "Alice",
            "age": 42
        })), Ok(()));

        assert_eq!(value.validate(&json!("")), Err(SchemaTypeValidationError::NotAnObject));

        assert_eq!(value.validate(&json!({
            "age": 42
        })), Err(SchemaTypeValidationError::MissingObjectKey("name".to_string())));

        assert_eq!(value.validate(&json!({
            "name": 10,
            "age": 42
        })), Err(
            SchemaTypeValidationError::BasicTypeValidationError(
                BasicTypeValidationError::IncorrectType(
                    BasicType::String,
                    json!(10)
                )
            )
        ));
    }

    #[test]
    fn optional_type_in_object_is_resolved_correctly() {
        let advanced_type: SchemaType = serde_json::from_value(json!({
            "name": {
                "$": "optional",
                "type": "string"
            }
        }))
            .unwrap();

        assert_eq!(advanced_type.validate(&json!({})), Ok(()));
    }

    #[test]
    fn incorrect_optional_type_in_object_returns_an_error() {
        let advanced_type: SchemaType = serde_json::from_value(json!({
            "name": {
                "$": "optional",
                "type": "string"
            }
        }))
            .unwrap();

        assert!(advanced_type.validate(&json!({
            "name": 10,
        })).is_err());
    }
}
