use serde::{Deserialize, Deserializer};
use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SchemaValidationError {
    #[error("invalid schema value")]
    InvalidSchemaValue,
}

/// A schema encapsulates multiple version of the schema which are updated through migrations.
#[derive(Debug)]
pub struct Schema {
    inner: Value,
}

impl<'de> Deserialize<'de> for Schema {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let inner = Value::deserialize(deserializer)?;

        Ok(Schema { inner })
    }
}

#[cfg(test)]
mod tests {
    use crate::schema::Schema;
    use serde_json::json;

    #[test]
    fn root_type_schema_validates_value_correctly() {
        let _schema: Schema = serde_json::from_value(json!("string")).unwrap();
    }
}
