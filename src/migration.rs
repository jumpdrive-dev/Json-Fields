use serde_json::Value;
use thiserror::Error;
use crate::migration::json_path::JsonPathError;
use crate::migration::operation::Operation;
use crate::migration::operation_kind::OperationKind;
use crate::migration::resolve_path::{PathResolveError, resolve_path, resolve_path_mut};
use crate::migration::set_path::{SetPath, SetPathError};

pub mod operation;
pub mod operation_kind;
mod resolve_path;
mod set_path;
mod json_path;

#[derive(Debug, Error)]
pub enum MigrationError {
    #[error("Failed to resolve path in migration: {0}")]
    PathError(#[from] JsonPathError),

    #[error("Failed to set path in migration: {0}")]
    SetError(#[from] SetPathError),

    #[error("Cannot delete a value on {0}")]
    CannotDeleteOn(Value),

    #[error("Expected an index to, but got {0} instead")]
    NotAnIndex(String),
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
                OperationKind::Delete => {
                    let last = op.target.clone_last();
                    let parent = op.target.parent();

                    let Some(parent) = parent else {
                        working_copy = Value::Null;
                        continue;
                    };

                    let last = last.expect("Last should never be None if the there is a parent");

                    let target_value = parent.resolve_mut(&mut working_copy)?;

                    match target_value {
                        Value::Array(array) => {
                            let index = last
                                .parse()
                                .map_err(|_| MigrationError::NotAnIndex(last))?;

                            array.remove(index);
                        }
                        Value::Object(object) => {
                            object.remove(&last);
                        }
                        _ => return Err(MigrationError::CannotDeleteOn(target_value.clone())),
                    }
                }
                OperationKind::Copy { new_path: copy_to } => {
                    let value = op.target.resolve(&working_copy)?.clone();
                    working_copy.set_path(copy_to, value)?;
                }
            }
        }

        Ok(working_copy)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use serde_json::json;
    use crate::migration::json_path::JsonPath;
    use crate::migration::Migration;
    use crate::migration::operation::Operation;
    use crate::migration::operation_kind::OperationKind;

    #[test]
    fn key_can_be_renamed() {
        let from = json!({ "a": 10 });
        let to = json!({ "b": 10 });

        let migration = Migration::with_operations([
            Operation::new(JsonPath::from_str("$.a").unwrap(), OperationKind::Copy {
                new_path: JsonPath::from_str("$.b").unwrap(),
            }),
            Operation::new(JsonPath::from_str("$.a").unwrap(), OperationKind::Delete),
        ]);

        let renamed = migration.migrate(from);

        assert_eq!(renamed.unwrap(), to);
    }
}
