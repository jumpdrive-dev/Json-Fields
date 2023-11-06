use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use crate::schema_type::{SchemaType, SchemaTypeValidationError};
use crate::traits::validator::Validator;
use crate::shared::default_true;

#[derive(Debug, PartialEq, Error)]
pub enum ArrayTypeError {
    #[error("Expected an array, but got something else")]
    NotAnArray,

    #[error("The provided array is empty, but should contain at least one item")]
    RequireFilled,

    #[error("{0}")]
    SchemaTypeValidationError(Box<SchemaTypeValidationError>),
}

impl From<SchemaTypeValidationError> for ArrayTypeError {
    fn from(value: SchemaTypeValidationError) -> Self {
        ArrayTypeError::SchemaTypeValidationError(Box::new(value))
    }
}

/// Checks for a variable length array that all match the given type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrayType {
    /// If this is set to true, the array should have at least one item.
    #[serde(default = "default_true")]
    pub require_filled: bool,

    /// The shape that all items in the array should have.
    pub items: Box<SchemaType>,
}

impl Display for ArrayType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "array with items: ")?;
        self.items.fmt(f)?;

        Ok(())
    }
}

impl Validator for ArrayType {
    type E = ArrayTypeError;

    fn validate(&self, value: &Value) -> Result<(), Self::E> {
        let Value::Array(items) = value else {
            return Err(ArrayTypeError::NotAnArray);
        };

        if items.is_empty() && self.require_filled {
            return Err(ArrayTypeError::RequireFilled);
        }

        for item in items {
            self.items.validate(item)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::schema_type::advanced_type::array_type::{ArrayType, ArrayTypeError};
    use crate::schema_type::basic_type::BasicType;
    use crate::schema_type::SchemaType;
    use crate::traits::validator::Validator;

    #[test]
    fn array_type_is_resolved_correctly() {
        let array_type = ArrayType {
            require_filled: true,
            items: Box::new(SchemaType::Basic(BasicType::String)),
        };

        assert_eq!(array_type.validate(&json!([
            "Alice",
            "Bob"
        ])), Ok(()));
    }

    #[test]
    fn require_filled_is_resolved_correctly() {
        let array_type = ArrayType {
            require_filled: true,
            items: Box::new(SchemaType::Basic(BasicType::String)),
        };

        assert_eq!(array_type.validate(&json!([
            ""
        ])), Ok(()));

        assert_eq!(array_type.validate(&json!([])), Err(ArrayTypeError::RequireFilled));
    }

    #[test]
    fn any_incorrect_type_returns_an_error() {
        let array_type = ArrayType {
            require_filled: true,
            items: Box::new(SchemaType::Basic(BasicType::String)),
        };

        assert!(array_type.validate(&json!([
            "Alice",
            "Bob",
            10
        ])).is_err());
    }
}
