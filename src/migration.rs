use serde_json::Value;
use crate::migration::operation::Operation;

pub mod operation;
pub mod operation_kind;
pub mod resolve_path;

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

    pub fn migrate(&self, value: Value) -> Result<Value, ()> {
        todo!()
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
        ]);

        let renamed = migration.migrate(from);

        assert_eq!(renamed.unwrap(), to);
    }
}
