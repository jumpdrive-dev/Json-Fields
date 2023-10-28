pub mod advanced_string_type;
pub mod any_of_type;
pub mod optional_type;

use std::fmt::{Display, Formatter, Pointer};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use crate::schema_type::advanced_type::advanced_string_type::{AdvancedStringType, StringValidationError};
use crate::schema_type::{SchemaType, SchemaTypeValidationError};
use crate::schema_type::advanced_type::any_of_type::{AnyOfType, AnyOfTypeError};
use crate::schema_type::advanced_type::optional_type::OptionalType;
use crate::traits::validator::Validator;

/// Types that require more configuration than just checking if the type matches.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "$", rename_all = "camelCase")]
pub enum AdvancedType {
    String(AdvancedStringType),
    AnyOf(AnyOfType),
    FixedArray,
    VariableArray,
    Optional(OptionalType),
}

impl Display for AdvancedType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AdvancedType::String(advanced_string_type) => advanced_string_type.fmt(f),
            AdvancedType::AnyOf(advanced_enum_type) => advanced_enum_type.fmt(f),
            AdvancedType::FixedArray => {
                todo!()
            }
            AdvancedType::VariableArray => {
                todo!()
            }
            AdvancedType::Optional(_) => {
                todo!()
            }
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
            AdvancedType::Optional(optional_type) => Ok(optional_type.validate(value)?),
            AdvancedType::AnyOf(advanced_enum) => Ok(advanced_enum.validate(value)?),
            AdvancedType::FixedArray => {
                todo!()
            }
            AdvancedType::VariableArray => {
                todo!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::schema_type::advanced_type::advanced_string_type::AdvancedStringType;
    use crate::schema_type::advanced_type::AdvancedType;
    use crate::schema_type::advanced_type::any_of_type::AnyOfType;
    use crate::schema_type::advanced_type::optional_type::OptionalType;
    use crate::schema_type::basic_type::BasicType;
    use crate::schema_type::SchemaType;

    #[test]
    fn advanced_string_type_is_deserialized_correctly() {
        let advanced_type: AdvancedType = serde_json::from_value(json!({
            "$": "string",
            "requireFilled": false,
            "minLength": 10,
            "maxLength": 20,
        }))
            .unwrap();

        assert_eq!(advanced_type, AdvancedType::String(AdvancedStringType {
            require_filled: false,
            min_length: Some(10),
            max_length: Some(20),
        }));
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

        assert_eq!(advanced_type, AdvancedType::AnyOf(AnyOfType {
            variants: vec![
                SchemaType::Basic(BasicType::String),
                SchemaType::Basic(BasicType::Number),
            ],
        }));
    }

    #[test]
    fn optional_type_is_deserialized_correctly() {
        let advanced_type: AdvancedType = serde_json::from_value(json!({
            "$": "optional",
            "type": "string"
        }))
            .unwrap();

        assert_eq!(advanced_type, AdvancedType::Optional(OptionalType {
            kind: Box::new(SchemaType::Basic(BasicType::String))
        }));
    }
}
