use serde::{Deserialize, Serialize};
use crate::migration::operation_kind::OperationKind;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    pub target: String,

    #[serde(flatten)]
    pub op: OperationKind,
}

impl Operation {
    pub fn new(target_path: impl Into<String>, kind: OperationKind) -> Self {
        Self {
            target: target_path.into(),
            op: kind,
        }
    }
}
