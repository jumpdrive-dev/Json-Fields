pub mod advanced_string_type;
pub mod any_of_type;
pub mod optional_type;
pub mod tuple_type;
pub mod array_type;
pub mod object_type;

use crate::schema_type::advanced_type::advanced_string_type::{
    AdvancedStringType, StringValidationError,
};
use crate::schema_type::advanced_type::any_of_type::{AnyOfType, AnyOfTypeError};
use crate::schema_type::advanced_type::optional_type::OptionalType;
use crate::schema_type::SchemaTypeValidationError;
use crate::traits::validator::Validator;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{Debug, Display, Formatter};
use thiserror::Error;
use crate::schema_type::advanced_type::array_type::{ArrayType, ArrayTypeError};
use crate::schema_type::advanced_type::object_type::{ObjectType, ObjectTypeError};
use crate::schema_type::advanced_type::tuple_type::{TupleError, TupleType};

/// Types that require more configuration than just checking if the type matches.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "$", rename_all = "camelCase")]
pub enum AdvancedType {
    String(AdvancedStringType),
    AnyOf(AnyOfType),
    Tuple(TupleType),
    Array(ArrayType),
    Object(ObjectType),
    Optional(OptionalType),
}

impl Display for AdvancedType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AdvancedType::String(advanced_string_type) => Display::fmt(advanced_string_type, f),
            AdvancedType::AnyOf(advanced_enum_type) => Display::fmt(advanced_enum_type, f),
            AdvancedType::Tuple(tuple_type) => Display::fmt(tuple_type, f),
            AdvancedType::Array(array_type) => Display::fmt(array_type, f),
            AdvancedType::Object(object_type) => Display::fmt(object_type, f),
            AdvancedType::Optional(optional_type) => Display::fmt(optional_type, f),
        }
    }
}

#[derive(Debug, PartialEq, Error)]
pub enum AdvancedTypeValidationError {
    #[error("{0}")]
    StringValidationError(#[from] StringValidationError),

    #[error("{0}")]
    AnyOfError(#[from] AnyOfTypeError),

    #[error("{0}")]
    TupleError(#[from] TupleError),

    #[error("{0}")]
    ArrayError(#[from] ArrayTypeError),

    #[error("{0}")]
    ObjectError(#[from] ObjectTypeError),

    #[error("{0}")]
    SchemaTypeValidationError(Box<SchemaTypeValidationError>),
}

impl From<SchemaTypeValidationError> for AdvancedTypeValidationError {
    fn from(value: SchemaTypeValidationError) -> Self {
        AdvancedTypeValidationError::SchemaTypeValidationError(Box::new(value))
    }
}

impl Validator for AdvancedType {
    type E = AdvancedTypeValidationError;

    fn validate(&self, value: &Value) -> Result<(), Self::E> {
        match self {
            AdvancedType::String(advanced_string) => Ok(advanced_string.validate(value)?),
            AdvancedType::AnyOf(advanced_enum) => Ok(advanced_enum.validate(value)?),
            AdvancedType::Tuple(fixed_array_type) => Ok(fixed_array_type.validate(value)?),
            AdvancedType::Array(array_type) => Ok(array_type.validate(value)?),
            AdvancedType::Object(object_type) => Ok(object_type.validate(value)?),
            AdvancedType::Optional(optional_type) => Ok(optional_type.validate(value)?),
        }
    }
}

impl From<AdvancedStringType> for AdvancedType {
    fn from(value: AdvancedStringType) -> Self {
        AdvancedType::String(value)
    }
}

impl From<AnyOfType> for AdvancedType {
    fn from(value: AnyOfType) -> Self {
        AdvancedType::AnyOf(value)
    }
}

impl From<TupleType> for AdvancedType {
    fn from(value: TupleType) -> Self {
        AdvancedType::Tuple(value)
    }
}

impl From<ArrayType> for AdvancedType {
    fn from(value: ArrayType) -> Self {
        AdvancedType::Array(value)
    }
}

impl From<ObjectType> for AdvancedType {
    fn from(value: ObjectType) -> Self {
        AdvancedType::Object(value)
    }
}

impl From<OptionalType> for AdvancedType {
    fn from(value: OptionalType) -> Self {
        AdvancedType::Optional(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::schema_type::advanced_type::advanced_string_type::AdvancedStringType;
    use crate::schema_type::advanced_type::any_of_type::AnyOfType;
    use crate::schema_type::advanced_type::optional_type::OptionalType;
    use crate::schema_type::advanced_type::AdvancedType;
    use crate::schema_type::basic_type::BasicType;
    use crate::schema_type::SchemaType;
    use serde_json::json;
    use crate::schema_type::advanced_type::tuple_type::TupleType;

    #[test]
    fn advanced_string_type_is_deserialized_correctly() {
        let advanced_type: AdvancedType = serde_json::from_value(json!({
            "$": "string",
            "requireFilled": false,
            "minLength": 10,
            "maxLength": 20,
        }))
        .unwrap();

        assert_eq!(
            advanced_type,
            AdvancedType::String(AdvancedStringType {
                require_filled: false,
                min_length: Some(10),
                max_length: Some(20),
            })
        );
    }

    #[test]
    fn any_of_type_is_deserialized_correctly() {
        let advanced_type: AdvancedType = serde_json::from_value(json!({
            "$": "anyOf",
            "variants": [
                "string",
                "number",
            ],
        }))
        .unwrap();

        assert_eq!(
            advanced_type,
            AnyOfType {
                variants: vec![
                    BasicType::String.into(),
                    BasicType::Number.into(),
                ],
            }.into()
        );
    }

    #[test]
    fn tuple_type_is_deserialized_correctly() {
        let advanced_type: AdvancedType = serde_json::from_value(json!({
            "$": "tuple",
            "items": [
                "string",
                "number",
            ],
        }))
            .unwrap();

        assert_eq!(
            advanced_type,
            TupleType {
                items: vec![
                    BasicType::String.into(),
                    BasicType::Number.into(),
                ],
            }.into()
        );
    }

    #[test]
    fn optional_type_is_deserialized_correctly() {
        let advanced_type: AdvancedType = serde_json::from_value(json!({
            "$": "optional",
            "type": "string"
        }))
        .unwrap();

        assert_eq!(
            advanced_type,
            OptionalType {
                kind: Box::new(BasicType::String.into())
            }.into()
        );
    }
}
