use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use thiserror::Error;
use crate::traits::validator::Validator;

#[derive(Debug, Error)]
pub enum SchemaValidationError {
    #[error("invalid schema value")]
    InvalidSchemaValue,
}

#[derive(Debug)]
pub struct Schema {
    inner: Value,
}

impl<'de> Deserialize<'de> for Schema {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let inner = Value::deserialize(deserializer)?;

        Ok(Schema {
            inner,
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::schema::Schema;

    #[test]
    fn root_type_schema_validates_value_correctly() {
        let schema: Schema = serde_json::from_value(json!("string"))
            .unwrap();
    }
}
