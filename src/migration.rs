use serde_json::Value;
use thiserror::Error;
use crate::migration::operation::Operation;
use crate::migration::operation_kind::OperationKind;
use crate::migration::resolve_path::{PathResolveError, resolve_path, resolve_path_mut};
use crate::migration::set_path::{set_path, SetPathError};

pub mod operation;
pub mod operation_kind;
mod resolve_path;
mod set_path;

#[derive(Debug, Error)]
pub enum MigrationError {
    #[error("Failed to resolve path in migration: {0}")]
    PathError(#[from] PathResolveError),

    #[error("Failed to set: {0}")]
    SetError(#[from] SetPathError),
}

pub struct Migration {
    operations: Vec<Operation>,
}

impl Migration {
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    pub fn with_operations(operations: impl IntoIterator<Item = Operation>) -> Self {
        Self {
            operations: operations.into_iter().collect(),
        }
    }

    pub fn push_operation(&mut self, operation: Operation) {
        self.operations.push(operation);
    }

    pub fn push_operations(&mut self, operations: impl Iterator<Item = Operation>) {
        self.operations.extend(operations);
    }

    pub fn migrate(&self, value: Value) -> Result<Value, MigrationError> {
        let mut working_copy = value.clone();

        for op in &self.operations {
            match &op.op {
                OperationKind::Delete => {}
                OperationKind::Copy { new_path } => {
                    let target_value = resolve_path(&op.target, &working_copy)?.clone();
                    set_path(new_path, &mut working_copy, target_value)?;
                }
            }
        }

        Ok(working_copy)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use crate::migration::Migration;
    use crate::migration::operation::Operation;
    use crate::migration::operation_kind::OperationKind;

    #[test]
    fn key_can_be_renamed() {
        let from = json!({ "a": 10 });
        let to = json!({ "b": 10 });

        let migration = Migration::with_operations([
            Operation::new("$.a", OperationKind::Copy {
                new_path: "$.b".into(),
            }),
            Operation::new("$.a", OperationKind::Delete),
        ]);

        let renamed = migration.migrate(from);

        assert_eq!(renamed.unwrap(), to);
    }
}
