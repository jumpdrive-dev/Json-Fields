use crate::migration::json_path::JsonPath;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum OperationKind {
    Delete,
    Copy { new_path: JsonPath },
}
