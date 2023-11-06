pub mod schema_change;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use thiserror::Error;
use crate::schema::schema_change::SchemaChange;
use crate::schema_type::SchemaType;

#[derive(Debug, Error)]
pub enum SchemaValidationError {
    #[error("invalid schema value")]
    InvalidSchemaValue,
}

/// A schema encapsulates multiple version of the schema which are updated through migrations.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    version: u32,
    initial: SchemaType,
    changes: Vec<SchemaChange>,
}

impl Schema {
    pub fn add_change(&mut self, change: SchemaChange) {
        self.version += 1;
        self.changes.push(change);
    }
}

impl From<SchemaType> for Schema {
    fn from(value: SchemaType) -> Self {
        Schema {
            version: 0,
            initial: value,
            changes: vec![],
        }
    }
}
