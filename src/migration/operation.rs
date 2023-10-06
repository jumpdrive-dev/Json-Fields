use crate::migration::operation_kind::OperationKind;

pub struct Operation {
    target_path: String,
    kind: OperationKind,
}

impl Operation {
    pub fn new(target_path: impl Into<String>, kind: OperationKind) -> Self {
        Self {
            target_path: target_path.into(),
            kind,
        }
    }
}
