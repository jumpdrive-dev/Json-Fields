use crate::schema_type::advanced_type::{AdvancedType, AdvancedTypeValidationError};
use crate::schema_type::basic_type::{BasicType, BasicTypeValidationError};
use crate::traits::validator::Validator;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;
use crate::schema_type::advanced_type::advanced_string_type::AdvancedStringType;
use crate::schema_type::advanced_type::any_of_type::AnyOfType;
use crate::schema_type::advanced_type::array_type::ArrayType;
use crate::schema_type::advanced_type::object_type::ObjectType;
use crate::schema_type::advanced_type::optional_type::OptionalType;
use crate::schema_type::advanced_type::tuple_type::TupleType;
use crate::schema_type::field::Field;

pub mod advanced_type;
pub mod basic_type;
pub mod field;

#[derive(Debug, Error, PartialEq)]
pub enum SchemaTypeValidationError {
    #[error("{0}")]
    BasicTypeValidationError(#[from] BasicTypeValidationError),

    #[error("{0}")]
    AdvancedTypeValidationError(#[from] AdvancedTypeValidationError),
}

/// Root schema type that encompasses all the different types that can be validated.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum SchemaType {
    Basic(BasicType),
    Field(Field),
    Advanced(AdvancedType),
    Array((Box<SchemaType>,)),
    Tuple(Vec<SchemaType>),
    Object(HashMap<String, SchemaType>),
}

impl Display for SchemaType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SchemaType::Basic(basic_type) => Display::fmt(basic_type, f),
            SchemaType::Field(field) => Display::fmt(field, f),
            SchemaType::Advanced(advanced_type) => Display::fmt(advanced_type, f),
            SchemaType::Array(item) => {
                write!(f, "array filled with '{}'", item.0)
            }
            SchemaType::Tuple(items) => {
                let tuple_type = TupleType {
                    items: items.to_vec(),
                };

                Display::fmt(&tuple_type, f)
            }
            SchemaType::Object(_) => {
                write!(f, "object")
            }
        }
    }
}

impl Validator for SchemaType {
    type E = SchemaTypeValidationError;

    fn validate(&self, value: &Value) -> Result<(), Self::E> {
        match self {
            SchemaType::Basic(basic_type) => Ok(basic_type.validate(value)?),
            SchemaType::Field(field) => field.validate(value),
            SchemaType::Advanced(advanced_type) => Ok(advanced_type.validate(value)?),
            SchemaType::Array(item) => {
                let array_type = ArrayType {
                    require_filled: false,
                    items: item.0.clone(),
                };

                Ok(array_type.validate(value)
                    .map_err(|error| SchemaTypeValidationError::AdvancedTypeValidationError(AdvancedTypeValidationError::ArrayError(error)))?)
            }
            SchemaType::Tuple(items) => {
                let tuple_type = TupleType {
                    items: items.to_vec()
                };

                Ok(tuple_type.validate(value)
                    .map_err(|error| SchemaTypeValidationError::AdvancedTypeValidationError(AdvancedTypeValidationError::TupleError(error)))?)
            }
            SchemaType::Object(map) => {
                let object_type = ObjectType {
                    object: map.clone(),
                };

                Ok(object_type.validate(value)
                    .map_err(|error| SchemaTypeValidationError::AdvancedTypeValidationError(AdvancedTypeValidationError::ObjectError(error)))?)
            }
        }
    }
}

impl From<BasicType> for SchemaType {
    fn from(value: BasicType) -> Self {
        SchemaType::Basic(value)
    }
}

impl From<AdvancedType> for SchemaType {
    fn from(value: AdvancedType) -> Self {
        SchemaType::Advanced(value)
    }
}

impl From<Field> for SchemaType {
    fn from(value: Field) -> Self {
        SchemaType::Field(value)
    }
}

impl From<(SchemaType,)> for SchemaType {
    fn from(value: (SchemaType, )) -> Self {
        SchemaType::Array((Box::new(value.0),))
    }
}

impl From<Vec<SchemaType>> for SchemaType {
    fn from(value: Vec<SchemaType>) -> Self {
        SchemaType::Tuple(value)
    }
}

impl From<HashMap<String, SchemaType>> for SchemaType {
    fn from(value: HashMap<String, SchemaType>) -> Self {
        SchemaType::Object(value)
    }
}

impl From<AdvancedStringType> for SchemaType {
    fn from(value: AdvancedStringType) -> Self {
        SchemaType::Advanced(value.into())
    }
}

impl From<AnyOfType> for SchemaType {
    fn from(value: AnyOfType) -> Self {
        SchemaType::Advanced(value.into())
    }
}

impl From<TupleType> for SchemaType {
    fn from(value: TupleType) -> Self {
        SchemaType::Advanced(value.into())
    }
}

impl From<ArrayType> for SchemaType {
    fn from(value: ArrayType) -> Self {
        SchemaType::Advanced(value.into())
    }
}

impl From<ObjectType> for SchemaType {
    fn from(value: ObjectType) -> Self {
        SchemaType::Advanced(value.into())
    }
}

impl From<OptionalType> for SchemaType {
    fn from(value: OptionalType) -> Self {
        SchemaType::Advanced(value.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::schema_type::advanced_type::advanced_string_type::AdvancedStringType;
    use crate::schema_type::basic_type::BasicTypeValidationError;
    use crate::schema_type::{AdvancedType, BasicType, SchemaType, SchemaTypeValidationError};
    use crate::traits::validator::Validator;
    use serde_json::json;
    use std::collections::HashMap;
    use crate::schema_type::advanced_type::AdvancedTypeValidationError;
    use crate::schema_type::advanced_type::object_type::ObjectTypeError;

    #[test]
    fn basic_schema_type_can_be_deserialized() {
        let value: SchemaType = serde_json::from_value(json!("string")).unwrap();

        assert_eq!(value, SchemaType::Basic(BasicType::String));
    }

    #[test]
    fn advanced_string_type_can_be_deserialized() {
        let value: SchemaType =
            serde_json::from_value(json!({ "$": "string", "minLength": 10 })).unwrap();

        assert_eq!(
            value,
            AdvancedStringType {
                min_length: Some(10),
                ..Default::default()
            }.into()
        );

        let result = serde_json::from_value::<SchemaType>(json!({ "minLength": 10 }));
        assert!(result.is_err());
    }

    #[test]
    fn nested_values_in_object_are_deserialized_correctly() {
        let value: SchemaType = serde_json::from_value(json!({
            "name": "string"
        }))
        .unwrap();

        assert_eq!(
            value,
            HashMap::from([(
                "name".to_string(),
                SchemaType::Basic(BasicType::String)
            )]).into()
        );
    }

    #[test]
    fn nested_values_in_array_are_deserialized_correctly() {
        let value: SchemaType = serde_json::from_value(json!(["string"])).unwrap();

        assert_eq!(
            value,
            SchemaType::Array((Box::new(SchemaType::Basic(BasicType::String)),))
        );
    }

    #[test]
    fn nested_values_in_tuple_are_deserialized_correctly() {
        let value: SchemaType = serde_json::from_value(json!(["string", "number"])).unwrap();

        assert_eq!(
            value,
            vec![
                SchemaType::Basic(BasicType::String),
                SchemaType::Basic(BasicType::Number),
            ].into()
        );
    }

    #[test]
    fn objects_are_validated_correctly() {
        let value: SchemaType = serde_json::from_value(json!({
            "name": "string",
            "age": "number",
        }))
        .unwrap();

        assert_eq!(
            value.validate(&json!({
                "name": "Alice",
                "age": 42
            })),
            Ok(())
        );

        assert_eq!(
            value.validate(&json!("")),
            Err(SchemaTypeValidationError::AdvancedTypeValidationError(
                AdvancedTypeValidationError::ObjectError(
                    ObjectTypeError::NotAnObject
                )
            ))
        );

        assert_eq!(
            value.validate(&json!({
                "age": 42
            })),
            Err(
                SchemaTypeValidationError::AdvancedTypeValidationError(
                    AdvancedTypeValidationError::ObjectError(
                        ObjectTypeError::MissingObjectKey(
                            "name".to_string()
                        )
                    )
                )
            )
        );

        assert!(
            value.validate(&json!({
                "name": 10,
                "age": 42
            })).is_err()
        );
    }

    #[test]
    fn optional_type_in_object_is_resolved_correctly() {
        let value: SchemaType = serde_json::from_value(json!({
            "name": {
                "$": "optional",
                "type": "string"
            }
        }))
        .unwrap();

        assert_eq!(value.validate(&json!({})), Ok(()));
    }

    #[test]
    fn incorrect_optional_type_in_object_returns_an_error() {
        let value: SchemaType = serde_json::from_value(json!({
            "name": {
                "$": "optional",
                "type": "string"
            }
        }))
        .unwrap();

        assert!(value
            .validate(&json!({
                "name": 10,
            }))
            .is_err());
    }

    #[test]
    fn unmatched_object_with_dollar_sign_key_is_deserialized_correctly() {
        let value: SchemaType = serde_json::from_value(json!({
            "$": "number",
            "name": "string"
        }))
            .unwrap();

        assert_eq!(
            value,
            HashMap::from([
                ("$".to_string(), SchemaType::Basic(BasicType::Number)),
                ("name".to_string(), SchemaType::Basic(BasicType::String)),
            ]).into()
        );
    }

    #[test]
    fn tuple_shorthand_is_resolved_correctly() {
        let value: SchemaType = serde_json::from_value(json!([
            "string",
            "number"
        ]))
            .unwrap();

        assert_eq!(value.validate(&json!([
            "",
            10
        ])), Ok(()));

        assert!(value.validate(&json!([
            "",
            ""
        ])).is_err());

        assert!(value.validate(&json!([""])).is_err());
        assert!(value.validate(&json!(["", 10, ""])).is_err());
    }
}
