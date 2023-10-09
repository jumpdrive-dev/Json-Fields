use serde::Serialize;
use crate::migration::json_path::JsonPathError;
use crate::migration::operation::Operation;
use crate::migration::operation_kind::{OperationError, OperationKind};
use serde_json::Value;
use thiserror::Error;

use crate::migration::set_path::{SetPath, SetPathError};

mod json_path;
pub mod operation;
pub mod operation_kind;
mod set_path;

#[derive(Debug, Error)]
pub enum MigrationError {
    #[error("Failed to perform operation: {0}")]
    OperationError(#[from] OperationError),
}

#[derive(Default, Serialize)]
pub struct Migration {
    operations: Vec<Operation>,
}

impl Migration {
    pub fn new() -> Self {
        Self::default()
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

        for operation in &self.operations {
            operation.op.apply(&operation.target, &mut working_copy)?;
        }

        Ok(working_copy)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::migration::json_path::JsonPath;
    use crate::migration::operation::Operation;
    use crate::migration::operation_kind::OperationKind;
    use crate::migration::Migration;
    use serde_json::{json, Number, Value};
    use crate::migration::operation_kind::value_source::ValueSource;

    #[test]
    fn key_can_be_renamed() {
        let from = json!({ "a": 10 });
        let to = json!({ "b": 10 });

        let migration = Migration::with_operations([
            Operation::new(
                JsonPath::from(["a"]),
                OperationKind::Rename { new_path: JsonPath::from(["b"]), },
            ),
        ]);

        let transformed = migration.migrate(from);

        assert_eq!(transformed.unwrap(), to);
    }

    #[test]
    fn set_value_directly() {
        let from = json!({});
        let to = json!({ "a": 10 });

        let migration = Migration::with_operations([
            Operation::new(
                JsonPath::from(["a"]),
                OperationKind::Set { value: Some(Value::Number(Number::from(10))), source: None, },
            ),
        ]);

        let transformed = migration.migrate(from);

        assert_eq!(transformed.unwrap(), to);
    }

    #[test]
    fn set_value_from_path_source() {
        let from = json!({ "a": 10 });
        let to = json!({ "a": 10, "b": 10 });

        let migration = Migration::with_operations([
            Operation::new(
                JsonPath::from(["b"]),
                OperationKind::Set { value: None, source: Some(ValueSource::Path(JsonPath::from(["a"]))), },
            ),
        ]);

        let transformed = migration.migrate(from);

        assert_eq!(transformed.unwrap(), to);
    }

    #[test]
    fn set_value_from_array_source() {
        let from = json!({ "a": 10, "b": 20 });
        let to = json!({ "a": 10, "b": 20, "c": [10, 20] });

        let migration = Migration::with_operations([
            Operation::new(
                JsonPath::from(["c"]),
                OperationKind::set_source(ValueSource::Array(vec![
                    ValueSource::Path(JsonPath::from(["a"])),
                    ValueSource::Path(JsonPath::from(["b"])),
                ])),
            ),
        ]);

        let transformed = migration.migrate(from);

        assert_eq!(transformed.unwrap(), to);
    }

    #[test]
    fn set_value_from_object_source() {
        let from = json!({ "a": 10, "b": 20 });
        let to = json!({ "a": 10, "b": 20, "c": { "d": 10, "e": 20 } });

        let migration = Migration::with_operations([
            Operation::new(
                JsonPath::from(["c"]),
                OperationKind::set_source(ValueSource::Object(HashMap::from([
                    ("d".to_string(), ValueSource::Path(JsonPath::from(["a"]))),
                    ("e".to_string(), ValueSource::Path(JsonPath::from(["b"]))),
                ]))),
            ),
        ]);

        let transformed = migration.migrate(from);

        assert_eq!(transformed.unwrap(), to);
    }
}
