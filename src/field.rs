use serde::{Deserialize, Serialize};
use crate::schema_type::SchemaType;

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    #[serde(rename = "type")]
    kind: SchemaType,
    label: String,
    description: Option<String>,
}
