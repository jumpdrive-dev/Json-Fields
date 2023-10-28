use crate::schema_type::{SchemaType, SchemaTypeValidationError};
use crate::traits::validator::Validator;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Optional type that can either the the requested type or null. Parent types like objects should
/// also check for this specific for example to check whether to return an error if the required
/// key is missing. If you want to explicitly indicate either null or a type, use [AnyOfType]
/// instead.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptionalType {
    #[serde(rename = "type")]
    pub(crate) kind: Box<SchemaType>,
}

impl Validator for OptionalType {
    type E = SchemaTypeValidationError;

    fn validate(&self, value: &Value) -> Result<(), Self::E> {
        if let Value::Null = value {
            return Ok(());
        }

        self.kind.validate(value)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::schema_type::advanced_type::optional_type::OptionalType;
    use crate::schema_type::basic_type::{BasicType, BasicTypeValidationError};
    use crate::schema_type::{SchemaType, SchemaTypeValidationError};
    use crate::traits::validator::Validator;
    use serde_json::json;

    #[test]
    fn optional_type_is_resolved_correctly() {
        let optional_type = OptionalType {
            kind: Box::new(SchemaType::Basic(BasicType::String)),
        };

        assert_eq!(optional_type.validate(&json!("")), Ok(()));
        assert_eq!(optional_type.validate(&json!(null)), Ok(()));
        assert_eq!(
            optional_type.validate(&json!(10)),
            Err(SchemaTypeValidationError::BasicTypeValidationError(
                BasicTypeValidationError::IncorrectType(BasicType::String, json!(10))
            ))
        );
    }
}
