use crate::schema_type::SchemaType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    #[serde(rename = "type")]
    kind: SchemaType,
    label: String,
    description: Option<String>,
}
