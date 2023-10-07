use serde::{Deserialize, Serialize};
use crate::migration::json_path::JsonPath;
use crate::migration::operation_kind::OperationKind;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    pub target: JsonPath,

    #[serde(flatten)]
    pub op: OperationKind,
}

impl Operation {
    pub fn new(target_path: JsonPath, kind: OperationKind) -> Self {
        Self {
            target: target_path,
            op: kind,
        }
    }
}
