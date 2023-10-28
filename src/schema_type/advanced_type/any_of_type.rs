use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::schema_type::SchemaType;
use crate::traits::validator::Validator;

#[derive(Debug, PartialEq)]
pub struct AnyOfTypeError(pub Vec<SchemaType>);

impl Error for AnyOfTypeError {}

impl Display for AnyOfTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let one_of = self.0.iter()
            .map(|schema| schema.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "No matching variant. Expected one of: {}", one_of)
    }
}

/// Passes if the provided value matches any of the provided type conditions.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnyOfType {
    pub(crate) variants: Vec<SchemaType>,
}

impl Validator for AnyOfType {
    type E = AnyOfTypeError;

    fn validate(&self, value: &Value) -> Result<(), Self::E> {
        for variant in &self.variants {
            if let Ok(()) = variant.validate(value) {
                return Ok(())
            }
        }

        Err(AnyOfTypeError(self.variants.to_vec()))
    }
}

impl<const U: usize> From<[SchemaType; U]> for AnyOfType {
    fn from(value: [SchemaType; U]) -> Self {
        AnyOfType {
            variants: value.into_iter()
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::schema_type::advanced_type::any_of_type::{AnyOfType, AnyOfTypeError};
    use crate::schema_type::advanced_type::AdvancedType;
    use crate::schema_type::basic_type::BasicType;
    use crate::schema_type::SchemaType;
    use crate::traits::validator::Validator;

    #[test]
    fn single_any_of_variant_is_checked_correctly() {
        let enum_type = AnyOfType::from([
            SchemaType::Basic(BasicType::String),
        ]);

        assert_eq!(enum_type.validate(&json!("")), Ok(()));
        assert_eq!(enum_type.validate(&json!(10)), Err(AnyOfTypeError(vec![
            SchemaType::Basic(BasicType::String),
        ])));
        assert_eq!(enum_type.validate(&json!(null)), Err(AnyOfTypeError(vec![
            SchemaType::Basic(BasicType::String),
        ])));
    }

    #[test]
    fn multiple_any_of_variant_is_checked_correctly() {
        let enum_type = AnyOfType::from([
            SchemaType::Basic(BasicType::String),
            SchemaType::Basic(BasicType::Number),
        ]);

        assert_eq!(enum_type.validate(&json!("")), Ok(()));
        assert_eq!(enum_type.validate(&json!(10)), Ok(()));
        assert_eq!(enum_type.validate(&json!(null)), Err(AnyOfTypeError(vec![
            SchemaType::Basic(BasicType::String),
            SchemaType::Basic(BasicType::Number),
        ])));
    }

    #[test]
    fn nested_any_of_are_checked_correctly() {
        let enum_type = AnyOfType::from([
            SchemaType::Advanced(AdvancedType::AnyOf(AnyOfType::from([
                SchemaType::Basic(BasicType::String),
            ]))),
            SchemaType::Advanced(AdvancedType::AnyOf(AnyOfType::from([
                SchemaType::Basic(BasicType::Number),
            ]))),
        ]);

        assert_eq!(enum_type.validate(&json!("")), Ok(()));
        assert_eq!(enum_type.validate(&json!(10)), Ok(()));
        assert!(enum_type.validate(&json!(null)).is_err());
    }
}
