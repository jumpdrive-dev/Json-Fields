use serde::{Deserialize, Serialize};
use crate::migration::json_path::JsonPath;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum OperationKind {
    Delete,
    Copy { new_path: JsonPath, },
}
