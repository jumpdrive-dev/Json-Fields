use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use crate::schema_type::{SchemaType, SchemaTypeValidationError};
use crate::traits::validator::Validator;

#[derive(Debug, PartialEq, Error)]
pub enum TupleError {
    #[error("Expected an array, but got something else")]
    NotAnArray,

    #[error("Expected an array with {0} items, but got an array with {1} items")]
    IncorrectLength(usize, usize),

    #[error("{0}")]
    SchemaTypeValidationError(Box<SchemaTypeValidationError>),
}

impl From<SchemaTypeValidationError> for TupleError {
    fn from(value: SchemaTypeValidationError) -> Self {
        TupleError::SchemaTypeValidationError(Box::new(value))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TupleType {
    pub(crate) items: Vec<SchemaType>,
}

impl Display for TupleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let items = self
            .items
            .iter()
            .map(|schema| schema.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "tuple with items: {}", items)
    }
}

impl Validator for TupleType {
    type E = TupleError;

    fn validate(&self, value: &Value) -> Result<(), Self::E> {
        let Value::Array(value_items) = value else {
            return Err(TupleError::NotAnArray);
        };

        if value_items.len() != self.items.len() {
            return Err(TupleError::IncorrectLength(value_items.len(), self.items.len()));
        }

        for (i, schema) in self.items.iter().enumerate() {
            let item_value = value.get(i)
                .expect("Both vecs should be the same size, so this should never be None");

            schema.validate(item_value)?;
        }

        Ok(())
    }
}

impl<const U: usize> From<[SchemaType; U]> for TupleType {
    fn from(value: [SchemaType; U]) -> Self {
        TupleType {
            items: value.into_iter()
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::schema_type::advanced_type::tuple_type::{TupleError, TupleType};
    use crate::schema_type::basic_type::{BasicType, BasicTypeValidationError};
    use crate::schema_type::{SchemaType, SchemaTypeValidationError};
    use crate::traits::validator::Validator;

    #[test]
    fn all_items_are_validated_correctly() {
        let fixed_array_type = TupleType::from([
            SchemaType::Basic(BasicType::String),
            SchemaType::Basic(BasicType::Number),
        ]);

        assert_eq!(fixed_array_type.validate(&json!([
            "",
            10
        ])), Ok(()));

        assert_eq!(fixed_array_type.validate(&json!([
            "",
            ""
        ])), Err(TupleError::SchemaTypeValidationError(
            Box::new(
                SchemaTypeValidationError::BasicTypeValidationError(
                    BasicTypeValidationError::IncorrectType(
                        BasicType::Number,
                        json!("")
                    )
                )
            )
        )));
    }

    #[test]
    fn incorrect_number_of_items_returns_an_error() {
        let fixed_array_type = TupleType::from([
            SchemaType::Basic(BasicType::String),
            SchemaType::Basic(BasicType::Number),
        ]);

        assert_eq!(fixed_array_type.validate(&json!([""])), Err(TupleError::IncorrectLength(1, 2)));
        assert_eq!(fixed_array_type.validate(&json!(["", 10, ""])), Err(TupleError::IncorrectLength(3, 2)));
    }
}
