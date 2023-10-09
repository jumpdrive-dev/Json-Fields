pub mod value_source;

use std::mem;
use crate::migration::json_path::{JsonPath, JsonPathError};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use crate::migration::operation_kind::value_source::{ValueSource, ValueSourceError};
use crate::migration::set_path::{SetPath, SetPathError};

#[derive(Debug, Error)]
pub enum OperationError {
    #[error("Failed to resolve path: {0}")]
    PathError(#[from] JsonPathError),

    #[error("Failed to set path: {0}")]
    SetError(#[from] SetPathError),

    #[error("Failed to resolve source: {0}")]
    SourceError(#[from] ValueSourceError),

    #[error("Expected an index to, but got {0} instead")]
    NotAnIndex(String),

    #[error("Cannot delete a value on {0}")]
    CannotDeleteOn(Value),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum OperationKind {
    Delete,
    Copy { new_path: JsonPath },
    Rename { new_path: JsonPath },
    Set { value: Option<Value>, source: Option<ValueSource>, }
}

impl OperationKind {
    pub fn apply(&self, path: &JsonPath, working_value: &mut Value) -> Result<(), OperationError> {
        match self {
            OperationKind::Delete => {
                let last = path.clone_last();
                let parent = path.parent();

                let Some(parent) = parent else {
                    let _ = mem::replace(working_value, Value::Null);
                    return Ok(());
                };

                let last = last.expect("Last should never be None if the there is a parent");

                let target_value = parent.resolve_mut(working_value)?;

                match target_value {
                    Value::Array(array) => {
                        let index =
                            last.parse().map_err(|_| OperationError::NotAnIndex(last))?;

                        array.remove(index);
                    }
                    Value::Object(object) => {
                        object.remove(&last);
                    }
                    _ => return Err(OperationError::CannotDeleteOn(target_value.clone())),
                }
            }
            OperationKind::Copy { new_path: copy_to } => {
                let value = path.resolve(&working_value)?.clone();
                working_value.set_path(copy_to, value)?;
            }
            OperationKind::Rename { new_path } => {
                OperationKind::Copy { new_path: new_path.clone() }
                    .apply(path, working_value)?;

                OperationKind::Delete
                    .apply(path, working_value)?;
            }
            OperationKind::Set { value, source } => {
                if let Some(value) = value {
                    working_value.set_path(path, value.clone())?;
                }

                if let Some(source) = source {
                    let value = source.resolve(working_value)?;
                    working_value.set_path(path, value)?
                }
            }
        };

        Ok(())
    }

    pub fn set_value(value: Value) -> Self {
        OperationKind::Set { value: Some(value), source: None }
    }

    pub fn set_source(source: ValueSource) -> Self {
        OperationKind::Set { value: None, source: Some(source) }
    }
}
