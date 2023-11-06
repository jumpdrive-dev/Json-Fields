use serde::{Deserialize, Serialize};
use crate::schema_type::SchemaType;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaChange {
    new_schema: SchemaType,
}
