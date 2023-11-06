use std::fmt::{Debug, Display, Formatter};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::schema_type::{SchemaType, SchemaTypeValidationError};
use crate::traits::validator::Validator;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Field {
    #[serde(rename = "?")]
    field_type: Box<SchemaType>,

    label: String,
    hint: Option<String>,
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "field with ")?;
        Display::fmt(&self.field_type, f)
    }
}

impl Validator for Field {
    type E = SchemaTypeValidationError;

    fn validate(&self, value: &Value) -> Result<(), Self::E> {
        self.field_type.validate(value)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::schema_type::basic_type::BasicType;
    use crate::schema_type::field::Field;

    #[test]
    fn field_is_deserialized_correctly() {
        let result = serde_json::from_value::<Field>(json!({
            "?": "string",
            "label": "Name",
            "hint": "Your name"
        }));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Field {
            field_type: Box::new(BasicType::String.into()),
            label: "Name".to_string(),
            hint: Some("Your name".to_string()),
        });
    }
}
